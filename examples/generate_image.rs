use std::fs;
#[cfg(feature = "use-base64")]
use base64::Engine;
#[cfg(feature = "use-base64")]
use base64::{engine::general_purpose};
// function calling example
use gemini_rs::types::{FunctionDeclaration, InlineData, Part, Tools};
use serde::{Deserialize, Serialize};
/// Proposal : Image enum for processing in-line image data
/// [API Reference](https://ai.google.dev/gemini-api/docs/image-understanding#supported-formats)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InlineDataMimeTypeEnum {
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/webp")]
    ImageWebp,
    #[serde(rename = "image/heic")]
    ImageHeic,
    #[serde(rename = "image/heif")]
    ImageHeif
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini_rs::Client::instance();

    let mut chat_list = vec![Part::text("Remake Tree well")];
    #[cfg(feature = "use-base64")]{
        chat_list.push(Part::inline_data(
        "image/jpeg",
        &general_purpose::STANDARD.encode(
                &fs::read("examples/images/tree.jpg")?
            ),
        ));
    }

    let response = client
    .chat("gemini-2.5-flash-image-preview")
    .send_parted_messages(chat_list).await?;
    let image = response.candidates[0].content.parts.iter().filter(|a| {
        if let Part { inline_data: Some(InlineData { mime_type, data }), .. } = a {
            true
        } else {
            false
        }
    }).map(|img| {
        img.inline_data.as_ref().unwrap().data.clone()
    });

    #[cfg(feature = "use-base64")]{
        let image_data = general_purpose::STANDARD.decode(image.collect::<Vec<_>>()[0].as_str())?;
        let _ = fs::write("examples/images/response.png", &image_data)?;
    }
    #[cfg(not(feature = "use-base64"))]{
        let collected = image.collect::<Vec<_>>();
        let image_data = collected[0].as_str();
        let _ = fs::write("examples/images/response.json", image_data)?;
    }
    Ok(())
}