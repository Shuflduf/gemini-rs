use std::{
    fmt::Write as _,
    ops::{Deref, DerefMut},
    sync::{Arc, LazyLock},
};

use futures::FutureExt as _;
use reqwest::Method;
use secrecy::{ExposeSecret as _, SecretString};
use crate::{Chat, Error, Result, chat, types};

const BASE_URI: &str = "https://generativelanguage.googleapis.com";

pub struct Route<T> {
    client: Client,
    kind: T,
}

impl<T> Route<T> {
    fn new(client: &Client, kind: T) -> Self {
        Self {
            client: client.clone(),
            kind,
        }
    }
}

impl<T: Request> IntoFuture for Route<T> {
    type Output = Result<T::Model>;
    type IntoFuture = futures::future::BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        async move {
            let mut request = self
                .client
                .reqwest
                .request(T::METHOD, format!("{BASE_URI}/{self}"));

            if let Some(body) = self.kind.body() {
                request = request.json(&body);
            };
            let response = request.send().await?;

            let raw_json = response.text().await?;

            match serde_json::from_str::<types::ApiResponse<T::Model>>(&raw_json)? {
                types::ApiResponse::Ok(response) => Ok(response),
                types::ApiResponse::Err(api_error) => Err(Error::Gemini(api_error.error)),
            }
        }
        .boxed()
    }
}

impl<T> Deref for Route<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<T> DerefMut for Route<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl<T: Request> std::fmt::Display for Route<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = Formatter::new(fmt);
        self.kind.format_uri(&mut fmt)?;
        fmt.write_query_param("key", &self.client.key.expose_secret())
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

impl Deref for Client {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            inner: ClientInner::new(None),
        }
    }
}

impl Client {
    pub fn new(key: impl Into<SecretString>) -> Self {
        Self {
            inner: ClientInner::new(Some(key.into())),
        }
    }

    pub fn chat(&self, model: &str) -> Chat<chat::Text> {
        Chat::new(self, model)
    }

    pub fn models(&self) -> Route<Models> {
        Route::new(self, Models::default())
    }

    pub fn generate_content(&self, model: &str) -> Route<GenerateContent> {
        Route::new(self, GenerateContent::new(model.into()))
    }

    pub fn instance() -> Client {
        static STATIC_INSTANCE: LazyLock<Client> = LazyLock::new(Client::default);
        STATIC_INSTANCE.clone()
    }
}

pub struct GenerateContent {
    model: Box<str>,
    pub body: types::GenerateContent,
}

impl GenerateContent {
    pub fn new(model: Box<str>) -> Self {
        Self {
            model,
            body: types::GenerateContent::default(),
        }
    }

    pub fn config(&mut self, config: types::GenerationConfig) {
        self.body.generation_config = Some(config);
    }

    pub fn safety_settings(&mut self, safety_settings: Vec<types::SafetySettings>) {
        self.body.safety_settings = safety_settings;
    }

    pub fn system_instruction(&mut self, instruction: &str) {
        self.body.system_instruction = Some(types::SystemInstructionContent {
            parts: vec![types::SystemInstructionPart {
                text: Some(instruction.into()),
            }],
        });
    }
    pub fn tool_config(&mut self, conf: types::ToolConfig) {
        self.body.tool_config = Some(conf);
    }
    pub fn contents(&mut self, contents: Vec<types::Content>) {
        self.body.contents = contents;
    }

    pub fn message(&mut self, message: &str) {
        self.body.contents.push(types::Content {
            role: types::Role::User,
            parts: vec![types::Part::text(message)],
        });
    }
    pub fn tools(&mut self, tools: Vec<types::Tools>) {
        self.body.tools = tools;
    }
}

impl Request for GenerateContent {
    type Model = types::Response;
    type Body = types::GenerateContent;

    const METHOD: Method = Method::POST;

    fn format_uri(&self, fmt: &mut Formatter<'_, '_>) -> std::fmt::Result {
        fmt.write_str("v1beta/")?;
        fmt.write_str("models/")?;
        fmt.write_str(&self.model)?;
        fmt.write_str(":generateContent")
    }

    fn body(self) -> Option<Self::Body> {
        Some(self.body)
    }
}

#[derive(Default)]
pub struct Models {
    page_size: Option<usize>,
    page_token: Option<Box<str>>,
}

impl Models {
    pub fn page_size(&mut self, size: usize) {
        self.page_size = size.into();
    }

    pub fn page_token(&mut self, token: &str) {
        self.page_token = Some(Box::from(token));
    }
}

impl Request for Models {
    type Model = types::Models;
    type Body = ();

    const METHOD: Method = Method::GET;

    fn format_uri(&self, fmt: &mut Formatter<'_, '_>) -> std::fmt::Result {
        fmt.write_str("v1beta/")?;
        fmt.write_str("models")?;
        fmt.write_optional_query_param("page_size", self.page_size.as_ref())?;
        fmt.write_optional_query_param("page_token", self.page_token.as_ref())
    }
}

pub struct Formatter<'me, 'buffer> {
    formatter: &'me mut std::fmt::Formatter<'buffer>,
    is_first: bool,
}

impl<'buffer> Deref for Formatter<'_, 'buffer> {
    type Target = std::fmt::Formatter<'buffer>;

    fn deref(&self) -> &Self::Target {
        self.formatter
    }
}

impl DerefMut for Formatter<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.formatter
    }
}

impl<'me, 'buffer> Formatter<'me, 'buffer> {
    fn new(formatter: &'me mut std::fmt::Formatter<'buffer>) -> Self {
        Self {
            formatter,
            is_first: true,
        }
    }

    fn write_query_param(&mut self, key: &str, value: &impl std::fmt::Display) -> std::fmt::Result {
        if self.is_first {
            self.formatter.write_char('?')?;
            self.is_first = false;
        } else {
            self.formatter.write_char('&')?;
        }

        self.formatter.write_str(key)?;
        self.formatter.write_char('=')?;
        std::fmt::Display::fmt(value, self.formatter)
    }

    fn write_optional_query_param(
        &mut self,
        key: &str,
        value: Option<&impl std::fmt::Display>,
    ) -> std::fmt::Result {
        if let Some(value) = value {
            self.write_query_param(key, value)
        } else {
            Ok(())
        }
    }
}

pub struct ClientInner {
    reqwest: reqwest::Client,
    key: SecretString,
}

impl ClientInner {
    fn new(key: Option<SecretString>) -> Arc<Self> {
        Self {
            reqwest: reqwest::Client::new(),
            key: key
                .or_else(|| std::env::var("GEMINI_API_KEY").ok().map(Into::into))
                .expect("API key must be set either via argument or GEMINI_API_KEY environment variable"),
        }
        .into()
    }
}

pub trait Request: Send + Sized + 'static {
    type Model: serde::de::DeserializeOwned + Send + 'static;
    type Body: serde::ser::Serialize;

    const METHOD: Method;

    fn format_uri(&self, fmt: &mut Formatter<'_, '_>) -> std::fmt::Result;

    fn body(self) -> Option<Self::Body> {
        None
    }
}
