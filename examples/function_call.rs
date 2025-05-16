use std::{collections::BTreeMap, default, env};

use gemini_rs::types::{CodeExecutionTool, FunctionDeclaration, GoogleSearchTool, SafetySettings, Schema, ToolConfig, Tools, Type};
use serde_json::json;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        print!("args: {:#?}", args);

    let client = gemini_rs::Client::new(
        args.get(1).cloned().expect("API key argument missing")
    );
    
    let mut content = client.generate_content("gemini-2.5-flash-preview-04-17");
    content.tool_config(
        ToolConfig{
            function_calling_config: Some(
                gemini_rs::types::FunctionCallingConfig{
                    mode:Some(gemini_rs::types::FunctionCallingMode::Any),
                    allowed_function_names: Some(vec!["set_alarm".to_string()]),
                }
            ),

        }
    );
    let mut properties = BTreeMap::new();
    properties.insert(
        "time".to_string(),
        Schema {
            schema_type: Some(Type::String),
            format: None,
            title: None,
            description: Some("The time to set the alarm for.".to_string()),
            nullable: None,
            enum_values: None,
            max_items: None,
            min_items: None,
            properties: None,
            required: None,
            property_ordering: None,
            items: None,
        },
    );

    let parameters = Schema {
        schema_type: Some(Type::Object),
        properties: Some(properties),
        format: None,
        title: None,
        description: None,
        nullable: None,
        enum_values: None,
        max_items: None,
        min_items: None,
        required: Some(vec!["time".to_string()]),
        property_ordering: None,
        items: None,
    };

    let function_declaration = FunctionDeclaration {
        name: "set_alarm".to_string(),
        description: "Set an alarm for a specific time.".to_string(),
        parameters: Some(parameters),
        response: None,
    };


    content.body.tools = vec![
        Tools{ 
            //Set alarm 7:00 AM
            function_declarations: Some(vec![function_declaration]),
            google_search: None,
            //Search : What is the time in New York
            //google_search: Some(GoogleSearchTool{}),

            //What is the sum of the first 50 prime numbers? Generate and run code for the calculation, and make sure you get all 50. 
            code_execution: None,
            //Some(CodeExecutionTool{}), 
        }
    ];
    content.system_instruction("You are a helpful assistant. You can set alarms for the user.");
    content.config(gemini_rs::types::GenerationConfig {
        temperature: Some(0.7),
        top_p: Some(0.9),
        thinking_config: Some(gemini_rs::types::ThinkingConfig {
            thinking_budget:Some(10000),
            include_thoughts: Some(true),
        }),
        ..Default::default()
    });
    content.message("What is the sum of the first 50 prime numbers? Generate and run code for the calculation, and make sure you get all 50.");
    println!("Request: {:#?}", content.body);
    let response = content.await?;
    println!("{:#?}", response);
    Ok(())
}