use std::{default, env};

use gemini_rs::types::{CodeExecutionTool, FunctionDeclaration, FunctionParameters, GoogleSearchTool, SafetySettings, Schema, ToolConfig, Tools, Type};
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
    let mut fun_param = FunctionParameters::default();
    fun_param.properties = json!(
        {
            "time": {
                "type": "STRING",
                "description": "time to set alarm",
            },
            
        }
    );

    content.body.tools = vec![
        Tools{ 
            function_declarations: None, 
            google_search: None,
            //Search : What is the time in New York
            //google_search: Some(GoogleSearchTool{}),

            //What is the sum of the first 50 prime numbers? Generate and run code for the calculation, and make sure you get all 50. 
            code_execution: Some(CodeExecutionTool{}), 
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