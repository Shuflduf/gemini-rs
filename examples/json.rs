#![recursion_limit = "256"]

use gemini_rs::types::{Schema, Type};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Schema {
        schema_type: Some(Type::Object),
        properties: Some(
            [
                (
                    "name".to_string(),
                    Schema {
                        schema_type: Some(Type::String),
                        ..Default::default()
                    },
                ),
                (
                    "age".to_string(),
                    Schema {
                        schema_type: Some(Type::Integer),
                        ..Default::default()
                    },
                ),
                (
                    "election_percentage".to_string(),
                    Schema {
                        schema_type: Some(Type::Number),
                        ..Default::default()
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ),
        ..Default::default()
    };

    let list_person = Schema {
        schema_type: Some(Type::Array),
        items: Some(Box::new(person)),
        ..Default::default()
    };

    let response = gemini_rs::chat("gemini-1.5-flash")
        .to_json()
        .response_schema(list_person)
        .json::<serde_json::Value>("List some US presidents")
        .await?;

    println!("{:#?}", response);
    Ok(())
}
