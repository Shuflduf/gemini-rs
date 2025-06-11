use futures::StreamExt as _;
use gemini_rs::types::{CodeExecutionTool, Tools};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini_rs::Client::instance();
    let tools = vec![Tools {
        function_declarations: None,
        google_search: None,
        code_execution: Some(CodeExecutionTool {}),
    }];
    let mut req = client.stream_generate_content("gemini-2.5-flash-preview-05-20");
    req.message("what's the sum of the prime numbers between 1 and 100?");
    req.tools(tools);

    let stream = req.stream().await?;
    println!("Stream started...");

    stream
        .for_each(|chunk| async move {
            match chunk {
                Ok(chunk) => println!("Chunk: {:?}", chunk.candidates),
                Err(e) => eprintln!("Error in stream: {:?}", e),
            }
        })
        .await;

    Ok(())
}
