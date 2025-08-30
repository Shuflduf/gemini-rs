use std::marker::PhantomData;

use crate::{
    Client, Result,
    types::{self, Response, Part},
};

/// Simplest way to use gemini-rs, and covers 80% of use cases
pub struct Chat<T> {
    model: Box<str>,
    client: Client,
    system_instruction: Option<Box<str>>,
    safety_settings: Vec<types::SafetySettings>,
    history: Vec<types::Content>,
    config: Option<types::GenerationConfig>,
    phantom: PhantomData<T>,
}

impl<T> Chat<T> {
    pub fn new(client: &Client, model: &str) -> Self {
        Self {
            model: model.into(),
            client: client.clone(),
            system_instruction: None,
            safety_settings: Vec::new(),
            history: Vec::new(),
            config: None,
            phantom: PhantomData,
        }
    }

    pub fn config(&mut self) -> &types::GenerationConfig {
        self.config.get_or_insert_default()
    }

    pub fn to_json(mut self) -> Chat<Json> {
        self.config_mut().response_mime_type = Some("application/json".into());
        Chat {
            model: self.model,
            client: self.client,
            system_instruction: self.system_instruction,
            safety_settings: self.safety_settings,
            history: self.history,
            config: self.config,
            phantom: PhantomData,
        }
    }

    pub fn config_mut(&mut self) -> &mut types::GenerationConfig {
        self.config.get_or_insert_default()
    }

    pub fn history(&self) -> &[types::Content] {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut Vec<types::Content> {
        &mut self.history
    }

    pub fn safety_settings(&mut self, safety_settings: Vec<types::SafetySettings>) {
        self.safety_settings = safety_settings;
    }

    pub fn system_instruction(mut self, instruction: &str) -> Self {
        self.system_instruction = Some(Box::from(instruction));
        self
    }

    pub async fn generate_content(&mut self) -> Result<Response> {
        let mut generate_content = self.client.generate_content(&self.model);

        if let Some(system_instruction) = &self.system_instruction {
            generate_content.system_instruction(system_instruction);
        }

        if let Some(config) = &self.config {
            generate_content.config(config.clone());
        }

        generate_content.contents(self.history.clone());
        generate_content.safety_settings(self.safety_settings.clone());
        generate_content.await
    }

    pub async fn send_message(&mut self, message: &str) -> Result<Response> {
        self.history.push(types::Content {
            role: types::Role::User,
            parts: vec![types::Part::text(message)],
        });

        self.generate_content().await
    }
    pub async fn send_parted_messages(&mut self, parts: Vec<Part>) -> Result<Response> {
        self.history.push(types::Content {
            role: types::Role::User,
            parts,
        });

        self.generate_content().await
    }
}

impl Chat<Json> {
    pub fn response_schema(mut self, schema: types::Schema) -> Self {
        self.config_mut().response_schema = Some(schema);
        self
    }

    pub async fn json<T: serde::de::DeserializeOwned>(&mut self, message: &str) -> Result<T> {
        let response = self.send_message(message).await?;
        let json = format!("{response}");
        serde_json::from_str(&json).map_err(Into::into)
    }
}

pub struct Text {}

pub struct Json {}
