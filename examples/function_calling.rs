// function calling example
use gemini_rs::types::{FunctionDeclaration, Tools};
use serde_json::json;

#[tokio::main]
async fn main() {
    // 1. Build the function declaration
    let function_decs = vec![FunctionDeclaration {
        name: "set_theme".to_string(),
        description: "Set the theme of the application".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "theme": {
                    "type": "string",
                    "description": "The theme to set",
                    "enum": ["light", "dark"]
                }
            },
            "required": ["theme"]
        }),
    }];

    // 2. Wrap in Tools
    let tools = vec![Tools {
        function_declarations: function_decs,
    }];

    // 3. Get the client and builder
    let client = gemini_rs::Client::instance();
    let mut req = client.generate_content("gemini-2.0-flash");

    // 4. Use builder methods to set up the request
    req.tools(tools);
    req.message("Set the theme to dark.");

    // 5. Await the request to send it
    let response = req.await;

    // 6. Process response accordingly
    match response {
        Ok(resp) => match &resp.candidates[0].content.parts[0].function_call {
            Some(func) => set_theme(func.args["theme"].as_str().unwrap()),
            None => eprint!("Function not called"),
        },
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn set_theme(theme: &str) {
    println!("Setting application theme to {}", theme);
}
