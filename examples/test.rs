use gemini_rs::types::{CodeExecutionTool, Tools};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tools = vec![Tools {
        function_declarations: None,
        google_search: None,
        code_execution: Some(CodeExecutionTool {}),
    }];

    let client = gemini_rs::Client::instance();
    let mut req = client.generate_content("gemini-2.0-flash");
    req.body.tools = tools;
    req.message("Write a function to calculate the factorial of a number.");
    let response = req.await?;

    println!("Response: {:?}", response.candidates[0].content.parts);

    Ok(())
}
