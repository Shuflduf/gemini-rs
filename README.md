# gemini-rs

A fully-featured, idiomatic Rust client for Google's Gemini AI models.

## Overview

`gemini-rs` provides a robust, async-first interface to the Google Gemini API, supporting all major features:

- Text generation and chat conversations
- JSON-structured outputs
- Function calling
- Safety settings and content filtering
- System instructions
- Model configuration (temperature, tokens, etc.)
- Streaming responses

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gemini-rs = "0.1" # Replace with the actual version
tokio = { version = "1", features = ["full"] }
```

## Authentication

You need a Gemini API key, which can be provided in two ways:

- **Environment variable:** Set `GEMINI_API_KEY`
- **Programmatically:** Pass the key to `Client::new(api_key)`

## Quick Start

The simplest way to use the library is via the `chat` function, which covers 80% of use cases:

```rust
use gemini_rs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini_rs::Client::new("API key");
    let mut chat = client.chat("gemini-2.5-flash");
    let response = chat.send_message("Explain how AI works").await?;

    println!("{}", response);
    Ok(())
}
```

## Core Concepts

### The `Client` Type

The `Client` struct is the main entry point for advanced usage. It manages authentication and provides access to all API features.

- `Client::new(api_key)`: Create a client with a specific API key.
- `Client::instance()`: Get a singleton client using the environment variable.
- `client.chat(model)`: Start a chat session.
- `client.models()`: List available models.
- `client.generate_content(model)`: Low-level content generation.
- `client.stream_generate_content(model)`: Streaming content generation.

### The `Chat` Type

The `Chat` struct provides a high-level, conversational interface:

- `Chat::new(&client, model)`: Create a new chat session.
- `chat.send_message(msg)`: Send a message and get a response.
- `chat.generate_content()`: Generate a response using the current history.
- `chat.history() / chat.history_mut()`: Access or modify conversation history.
- `chat.config_mut()`: Access or modify generation settings.
- `chat.safety_settings(settings)`: Set safety filters.
- `chat.system_instruction(instruction)`: Set a system prompt.

#### JSON Mode

For structured outputs, convert to JSON mode:

```rust
let mut chat = gemini_rs::chat("gemini-2.0-pro").to_json();
let schema = /* ... */;
chat.response_schema(schema);
let result: MyStruct = chat.json("Give me a JSON object...").await?;
```

##### Defining Response Schemas

You can define complex JSON schemas using the `Schema` and `Type` system for precise control over the model's output format:

```rust
use gemini_rs::types::{Schema, Type};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a person schema
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
    
    // Define an array of persons
    let list_person = Schema {
        schema_type: Some(Type::Array),
        items: Some(Box::new(person)),
        ..Default::default()
    };
    
    let response = gemini_rs::chat("gemini-2.0-flash")
        .to_json()
        .response_schema(list_person)
        .json::<serde_json::Value>("List some US presidents")
        .await?;
        
    println!("{:#?}", response);
    Ok(())
}
```

### Streaming Responses

For large or incremental outputs, use streaming:

```rust
let mut stream = gemini_rs::client()
    .stream_generate_content("gemini-2.0-flash")
    .stream()
    .await?;
while let Some(response) = stream.next().await {
    let response = response?;
    println!("{}", response);
}
```

## JSON Schema Definition

The library provides a powerful `Schema` system for defining structured JSON outputs. This is particularly useful when you need the model to return data in a specific format.

### Basic Schema Types

The `Type` enum supports all JSON schema types:

```rust
use gemini_rs::types::{Schema, Type};

// String schema
let name_schema = Schema {
    schema_type: Some(Type::String),
    description: Some("Person's full name".to_string()),
    ..Default::default()
};

// Integer schema
let age_schema = Schema {
    schema_type: Some(Type::Integer),
    description: Some("Person's age in years".to_string()),
    ..Default::default()
};

// Number schema (for decimals)
let percentage_schema = Schema {
    schema_type: Some(Type::Number),
    description: Some("Percentage value".to_string()),
    ..Default::default()
};

// Boolean schema
let active_schema = Schema {
    schema_type: Some(Type::Boolean),
    description: Some("Whether the person is active".to_string()),
    ..Default::default()
};
```

### Object Schemas

Define complex objects with multiple properties:

```rust
use std::collections::HashMap;

let person_schema = Schema {
    schema_type: Some(Type::Object),
    description: Some("A person object".to_string()),
    properties: Some(
        [
            ("name".to_string(), Schema {
                schema_type: Some(Type::String),
                description: Some("Full name".to_string()),
                ..Default::default()
            }),
            ("age".to_string(), Schema {
                schema_type: Some(Type::Integer),
                description: Some("Age in years".to_string()),
                ..Default::default()
            }),
            ("email".to_string(), Schema {
                schema_type: Some(Type::String),
                description: Some("Email address".to_string()),
                ..Default::default()
            }),
        ]
        .into_iter()
        .collect(),
    ),
    required: Some(vec!["name".to_string(), "age".to_string()]),
    ..Default::default()
};
```

### Array Schemas

Define arrays of any schema type:

```rust
// Array of strings
let tags_schema = Schema {
    schema_type: Some(Type::Array),
    description: Some("List of tags".to_string()),
    items: Some(Box::new(Schema {
        schema_type: Some(Type::String),
        ..Default::default()
    })),
    ..Default::default()
};

// Array of objects (using the person schema from above)
let people_list_schema = Schema {
    schema_type: Some(Type::Array),
    description: Some("List of people".to_string()),
    items: Some(Box::new(person_schema)),
    ..Default::default()
};
```

### Complete Example: Complex Nested Schema

```rust
use gemini_rs::types::{Schema, Type};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define address schema
    let address_schema = Schema {
        schema_type: Some(Type::Object),
        properties: Some([
            ("street".to_string(), Schema {
                schema_type: Some(Type::String),
                ..Default::default()
            }),
            ("city".to_string(), Schema {
                schema_type: Some(Type::String),
                ..Default::default()
            }),
            ("zipcode".to_string(), Schema {
                schema_type: Some(Type::String),
                ..Default::default()
            }),
        ].into_iter().collect()),
        ..Default::default()
    };
    
    // Define person schema with nested address
    let person_schema = Schema {
        schema_type: Some(Type::Object),
        properties: Some([
            ("name".to_string(), Schema {
                schema_type: Some(Type::String),
                ..Default::default()
            }),
            ("age".to_string(), Schema {
                schema_type: Some(Type::Integer),
                ..Default::default()
            }),
            ("address".to_string(), address_schema),
            ("hobbies".to_string(), Schema {
                schema_type: Some(Type::Array),
                items: Some(Box::new(Schema {
                    schema_type: Some(Type::String),
                    ..Default::default()
                })),
                ..Default::default()
            }),
        ].into_iter().collect()),
        required: Some(vec!["name".to_string(), "age".to_string()]),
        ..Default::default()
    };
    
    // Use the schema
    let response = gemini_rs::chat("gemini-2.0-flash")
        .to_json()
        .response_schema(person_schema)
        .json::<serde_json::Value>("Create a person profile for John Doe")
        .await?;
        
    println!("{:#?}", response);
    Ok(())
}
```

### Schema Properties

The `Schema` struct supports many properties for fine-tuning:

- **`schema_type`** - The JSON type (String, Integer, Number, Boolean, Object, Array)
- **`description`** - Human-readable description of the field
- **`properties`** - For objects, a map of property name to schema
- **`items`** - For arrays, the schema of array elements
- **`required`** - For objects, list of required property names
- **`enum_values`** - Restrict values to a specific set
- **`format`** - Additional format constraints (e.g., "email", "date")

### Working with Typed Responses

You can deserialize responses directly to Rust structs:

```rust
#[derive(serde::Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    election_percentage: Option<f64>,
}

let response: Vec<Person> = gemini_rs::chat("gemini-2.0-flash")
    .to_json()
    .response_schema(list_person_schema)
    .json("List some US presidents with their ages")
    .await?;

for person in response {
    println!("{}: {} years old", person.name, person.age);
}
```

The library supports function calling, allowing Gemini to invoke tools and functions you define. This feature enables the model to perform actions beyond just generating text.

### Basic Function Calling Setup

```rust
use gemini_rs::{Client, types::*};
use serde_json::json;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key");
    let mut content = client.generate_content("gemini-2.0-pro");
    
    // Define function calling configuration
    let function_config = FunctionCallingConfig {
        mode: Some(FunctionCallingMode::Any),
        allowed_function_names: Some(vec!["set_alarm".to_string()]),
    };
    
    // Define the function schema
    let parameters = json!({
        "type": "object",
        "properties": {
            "time": {
                "type": "string",
                "description": "The time to set the alarm for (e.g., '7:00 AM')"
            }
        },
        "required": ["time"]
    });
    
    let function_declaration = FunctionDeclaration {
        name: "set_alarm".to_string(),
        description: "Set an alarm for a specific time.".to_string(),
        parameters: Some(parameters),
    };
    
    // Register the tool
    content.body.tools = vec![Tools {
        function_declarations: Some(vec![function_declaration]),
        google_search: None,
        code_execution: None,
    }];
    
    // Set function calling configuration
    content.body.tool_config = Some(ToolConfig {
        function_calling_config: Some(function_config),
    });
    
    // Set system instruction and send message
    content.system_instruction("You are a helpful assistant. You can set alarms for the user.");
    content.message("Set an alarm for 7:00 AM");
    
    let response = content.send().await?;
    println!("{:?}", response);
    
    Ok(())
}
```

### Function Calling Components

#### `FunctionCallingConfig`

Controls how and when functions can be called:

```rust
FunctionCallingConfig {
    mode: Some(FunctionCallingMode::Any),
    allowed_function_names: Some(vec!["set_alarm".to_string()]),
}
```

- **`mode: Any`** → Gemini can call any function from your list if appropriate
- **`allowed_function_names`** → Restricts which functions can be called (optional)

Available modes:
- `FunctionCallingMode::Any` - Model can call any available function
- `FunctionCallingMode::Auto` - Model automatically decides when to call functions
- `FunctionCallingMode::None` - Disables function calling

#### `FunctionDeclaration`

Defines a function that Gemini can call:

```rust
let function_declaration = FunctionDeclaration {
    name: "set_alarm".to_string(),
    description: "Set an alarm for a specific time.".to_string(),
    parameters: Some(parameters),
};
```

- **`name`** - The function identifier
- **`description`** - What the function does (helps the model decide when to use it)
- **`parameters`** - JSON Schema describing the function's input parameters

#### Parameter Schema

Define function parameters using JSON Schema:

```rust
let parameters = json!({
    "type": "object",
    "properties": {
        "time": {
            "type": "string",
            "description": "The time to set the alarm for"
        },
        "message": {
            "type": "string",
            "description": "Optional alarm message"
        }
    },
    "required": ["time"]
});
```

### Advanced Function Calling Example

```rust
use gemini_rs::{Client, types::*};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key");
    let mut content = client.generate_content("gemini-2.0-pro");
    
    // Define multiple functions
    let weather_function = FunctionDeclaration {
        name: "get_weather".to_string(),
        description: "Get current weather for a location".to_string(),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City or location name"
                }
            },
            "required": ["location"]
        })),
    };
    
    let alarm_function = FunctionDeclaration {
        name: "set_alarm".to_string(),
        description: "Set an alarm for a specific time".to_string(),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "time": {
                    "type": "string",
                    "description": "Time for the alarm (e.g., '7:00 AM')"
                },
                "label": {
                    "type": "string",
                    "description": "Optional label for the alarm"
                }
            },
            "required": ["time"]
        })),
    };
    
    // Configure function calling
    content.body.tool_config = Some(ToolConfig {
        function_calling_config: Some(FunctionCallingConfig {
            mode: Some(FunctionCallingMode::Auto),
            allowed_function_names: None, // Allow all functions
        }),
    });
    
    // Register tools
    content.body.tools = vec![Tools {
        function_declarations: Some(vec![weather_function, alarm_function]),
        google_search: None,
        code_execution: None,
    }];
    
    content.system_instruction("You are a helpful assistant with access to weather and alarm functions.");
    content.message("What's the weather like in Tokyo, and set an alarm for 8:30 AM");
    
    let response = content.send().await?;
    println!("{:?}", response);
    
    Ok(())
}
```

### Important Notes

- **Prompt Relevance**: The model will only call functions when the user's prompt is relevant to the function's purpose
- **Schema Consistency**: Ensure your function name, description, and parameters schema are consistent
- **Error Handling**: Function calls can fail; always handle responses appropriately
- **Multiple Functions**: You can define multiple functions and let the model choose which ones to use

## Advanced Features

- **Model Management:** `client.models()` returns a paginated list of available models.
- **Custom Generation Settings:** Use `chat.config_mut()` to set temperature, max tokens, stop sequences, etc.
- **Safety Settings:** Use `chat.safety_settings()` to control content filtering.
- **System Instructions:** Use `chat.system_instruction()` to set a system prompt for the model.
- **Function Calling:** Define tools and function schemas for advanced use cases (see `types::Tools` and `types::FunctionDeclaration`).

## Error Handling

All fallible operations return `gemini_rs::Result<T>`, which is an alias for `Result<T, gemini_rs::Error>`. The error type covers:

- `Serde`: Serialization/deserialization errors
- `Http`: Network or HTTP errors
- `Gemini`: API errors (with detailed status and message)

## Types and Data Structures

The library exposes all major Gemini API types under `gemini_rs::types`, including:

- `types::Response`, `types::Content`, `types::Part`
- `types::Model`, `types::Models`
- `types::GenerationConfig`, `types::SafetySettings`
- `types::Tools`, `types::FunctionDeclaration`, `types::FunctionCallingConfig`
- `types::Schema`, `types::Type` - For JSON schema definition
- And many more for advanced use

Refer to the source or API reference for details.

## Example: Advanced Chat

```rust
let mut chat = gemini_rs::chat("gemini-2.0-pro")
.system_instruction("You are a helpful assistant.");

chat.config_mut().temperature = Some(0.7);
chat.safety_settings(vec![
    gemini_rs::types::SafetySettings {
        category: gemini_rs::types::HarmCategory::HarmCategoryToxicity,
        threshold: gemini_rs::types::HarmBlockThreshold::BlockLowAndAbove,
    }
]);

let response = chat.send_message("Tell me a joke.").await?;
println!("{}", response);
```

## License

MIT