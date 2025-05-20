use std::collections::BTreeMap;

use gemini_rs::types::{
    FunctionCallingConfig, FunctionCallingMode, FunctionDeclaration, Schema, ToolConfig, Tools,
    Type,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini_rs::Client::instance();

    let mut content = client.generate_content("gemini-2.5-flash-preview-04-17");
    content.tool_config(ToolConfig {
        function_calling_config: Some(FunctionCallingConfig {
            mode: Some(FunctionCallingMode::Any),
            allowed_function_names: Some(vec!["set_alarm".to_string()]),
        }),
    });

    let mut properties = BTreeMap::new();
    properties.insert(
        "time".to_string(),
        Schema {
            schema_type: Some(Type::String),
            description: Some("The time to set the alarm for.".to_string()),
            ..Default::default()
        },
    );

    let parameters = json!({
        "type": "object",
        "properties": {
            "theme": {
                "type": "string",
                "description": "The theme to set",
                "enum": ["light", "dark"]
            }
        },
        "required": ["theme"]
    });

    let function_declaration = FunctionDeclaration {
        name: "set_alarm".to_string(),
        description: "Set an alarm for a specific time.".to_string(),
        parameters,
    };

    content.body.tools = vec![Tools {
        //Set alarm 7:00 AM
        function_declarations: Some(vec![function_declaration]),
        google_search: None,
        //Search : What is the time in New York
        //google_search: Some(GoogleSearchTool{}),

        //What is the sum of the first 50 prime numbers? Generate and run code for the calculation, and make sure you get all 50.
        code_execution: None,
        //Some(CodeExecutionTool{}),
    }];
    content.system_instruction("You are a helpful assistant. You can set alarms for the user.");
    content.message("What is the sum of the first 50 prime numbers? Generate and run code for the calculation, and make sure you get all 50.");
    println!("Request: {:#?}", content.body);
    let response = content.await?;
    println!("{:#?}", response);
    Ok(())
}
