use crate::client::{Formatter, Request};
use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::Poll,
};

use bytes::Bytes;
use futures::Stream;
use reqwest::Method;
use serde::ser::Error as _;

use crate::{
    Error, Result,
    client::{BASE_URI, GenerateContent, Route},
    types,
};

pub struct StreamGenerateContent(pub(crate) GenerateContent);

impl StreamGenerateContent {
    pub fn new(model: &str) -> Self {
        Self(GenerateContent::new(model.into()))
    }
}

impl Deref for Route<StreamGenerateContent> {
    type Target = GenerateContent;

    fn deref(&self) -> &Self::Target {
        &self.kind.0
    }
}

impl DerefMut for Route<StreamGenerateContent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind.0
    }
}

impl Route<StreamGenerateContent> {
    pub async fn stream(self) -> std::result::Result<RouteStream<StreamGenerateContent>, String> {
        let url = format!("{BASE_URI}/{}", self);
        let body = self.kind.body().clone();
        let mut request = self
            .client
            .reqwest
            .request(StreamGenerateContent::METHOD, url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;
        let stream = response.bytes_stream();

        Ok(RouteStream {
            phantom: std::marker::PhantomData,
            stream: Box::pin(stream),
            buffer: Vec::new(),
            pos: 0,
            state: ParseState::CannotAdvance,
        })
    }
}

pub struct RouteStream<T> {
    phantom: std::marker::PhantomData<T>,
    stream: Pin<Box<dyn Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send>>,
    buffer: Vec<u8>,
    pos: usize, // A cursor into the buffer.
    state: ParseState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    CannotAdvance,
    ReadingChars,
    ReadingValue,
    Finished,
}

#[derive(Debug)]
enum ParseOutcome {
    Ok(Option<types::Response>),
    Err(serde_json::Error),
    Eof,
}

impl RouteStream<StreamGenerateContent> {
    fn next_char_pos(&self) -> Option<usize> {
        self.buffer[self.pos..]
            .iter()
            .position(|&b| !b.is_ascii_whitespace())
            .map(|p| self.pos + p)
    }

    fn advance_next_char(&mut self) -> Option<u8> {
        self.pos = self.next_char_pos().unwrap_or(self.buffer.len());
        self.buffer.get(self.pos).copied()
    }

    fn current_char(&self) -> Option<u8> {
        self.buffer.get(self.pos).copied()
    }

    fn is_bridge_char(&self) -> bool {
        matches!(self.current_char(), Some(b'[') | Some(b','))
    }

    fn parse_chunk(&mut self) -> ParseOutcome {
        let mut de = serde_json::Deserializer::from_slice(&self.buffer[self.pos..])
            .into_iter::<types::Response>();
        match de.next() {
            Some(Ok(value)) => {
                self.pos += de.byte_offset();
                ParseOutcome::Ok(Some(value))
            }
            Some(Err(e)) if e.is_eof() => ParseOutcome::Eof,
            Some(Err(e)) => ParseOutcome::Err(e),
            None => ParseOutcome::Ok(None), // No more objects to read.
        }
    }

    fn try_parse_next(&mut self) -> Option<ParseOutcome> {
        match self.state {
            ParseState::CannotAdvance => None, // nothing to read
            ParseState::ReadingChars => {
                self.advance_next_char();
                if self.is_bridge_char() {
                    self.pos += 1; // Move past this '[' or ','
                    self.state = ParseState::ReadingValue;
                    None
                } else if let Some(b']') = self.current_char() {
                    // If we hit a ']', we can finish reading.
                    self.state = ParseState::Finished;
                    Some(ParseOutcome::Ok(None))
                } else {
                    None
                }
            }
            ParseState::ReadingValue => {
                self.advance_next_char();
                // Deserialize one object from our current position.
                let outcome = self.parse_chunk();
                match &outcome {
                    ParseOutcome::Ok(Some(_)) => {
                        self.state = ParseState::ReadingChars;
                    }
                    ParseOutcome::Ok(None) | ParseOutcome::Err(_) => {
                        self.state = ParseState::Finished;
                    }
                    ParseOutcome::Eof => {}
                };
                Some(outcome)
            }
            ParseState::Finished => None,
        }
    }
}

impl Stream for RouteStream<StreamGenerateContent> {
    type Item = Result<types::Response>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            // Housekeeping: drain the buffer if we've processed a lot.
            if self.pos > 2048 {
                let this_pos = self.pos;
                self.buffer.drain(..this_pos);
                self.pos = 0;
            }

            if let Some(outcome) = self.try_parse_next() {
                match outcome {
                    ParseOutcome::Ok(Some(response)) => return Poll::Ready(Some(Ok(response))),
                    ParseOutcome::Ok(None) if self.state == ParseState::Finished => {
                        return Poll::Ready(None);
                    }
                    ParseOutcome::Err(error) => return Poll::Ready(Some(Err(Error::Serde(error)))),
                    ParseOutcome::Eof => {} // Continue to read more data.
                    _ => {}
                }
            };

            // If we fell through, we need more data. Poll the underlying stream.
            match self.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    if self.buffer.is_empty() && !bytes.is_empty() {
                        self.state = ParseState::ReadingChars;
                    }
                    self.buffer.extend_from_slice(&bytes);
                    continue; // Loop again to process new data.
                }
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Err(e))) => {
                    self.state = ParseState::Finished;
                    return Poll::Ready(Some(Err(Error::Http(e))));
                }
                Poll::Ready(None) => {
                    // Underlying stream ended. Check if we're in a clean state.
                    if self.state != ParseState::Finished && self.pos < self.buffer.len() {
                        let msg =
                            format!("stream ended with unparsed data in state {:?}", self.state);
                        return Poll::Ready(Some(Err(serde_json::Error::custom(msg).into())));
                    }
                    self.state = ParseState::Finished;
                    return Poll::Ready(None);
                }
            }
        }
    }
}

impl Request for StreamGenerateContent {
    type Model = types::Response;
    type Body = types::GenerateContent;

    const METHOD: Method = Method::POST;

    fn format_uri(&self, fmt: &mut Formatter<'_, '_>) -> std::fmt::Result {
        fmt.write_str("v1beta/")?;
        fmt.write_str("models/")?;
        fmt.write_str(&self.0.model)?;
        fmt.write_str(":streamGenerateContent")
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.0.body.clone())
    }
}

impl std::fmt::Display for StreamGenerateContent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = Formatter::new(fmt);
        self.format_uri(&mut fmt)?;
        fmt.write_query_param("key", &self.0.model)
    }
}
