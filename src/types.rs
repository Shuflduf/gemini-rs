//! Contains every type used in the library

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The producer of the content
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ApiError),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub code: u16,
    pub message: String,
    pub status: Status,
    #[serde(default)]
    pub details: Vec<ErrorInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub metadata: Option<BTreeMap<String, String>>,
}

/// Common backend error codes you may encounter
///
/// Use the [API Reference](https://ai.google.dev/gemini-api/docs/troubleshooting#error-codes) for
/// troubleshooting steps
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// The request body is malformed
    InvalidArgument,
    /// Gemini API free tier is not available in your country. Please enable billing on your project in Google AI Studio.
    FailedPrecondition,
    /// Your API key doesn't have the required permissions.
    PermissionDenied,
    /// The requested resource wasn't found.
    NotFound,
    /// You've exceeded the rate limit.
    ResourceExhausted,
    /// An unexpected error occurred on Google's side.
    Internal,
    /// The service may be temporarily overloaded or down.
    Unavailable,
    /// The service is unable to finish processing within the deadline.
    DeadlineExceeded,
}

/// Response from [crate::Client::models] containing a paginated list of Models
///
/// [API Reference](https://ai.google.dev/api/models#response-body_1)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Models {
    pub models: Vec<Model>,
    pub next_page_token: Option<String>,
}

/// Information about a Generative Language Model
///
/// [API Reference](https://ai.google.dev/api/models#Model)
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub version: String,
    pub display_name: String,
    pub description: String,
    pub input_token_limit: i32,
    pub output_token_limit: i32,
    pub supported_generation_methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
}

/// Response from the model supporting multiple candidate responses
///
/// [API Reference](https://ai.google.dev/api/generate-content#generatecontentresponse)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub candidates: Vec<Candidate>,
    pub prompt_feedback: Option<PromptFeedback>,
    pub usage_metadata: Option<UsageMetadata>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.candidates[0].content.parts[0]
                .text
                .as_deref()
                .unwrap_or_default(),
        )
    }
}

/// Metadata on the generation request's token usage
///
/// [API Reference](https://ai.google.dev/api/generate-content#UsageMetadata)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: u64,
    pub candidates_token_count: u64,
}

/// A response candidate generated from the model
///
/// [API Reference](https://ai.google.dev/api/generate-content#candidate)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: Option<FinishReason>,
    pub index: Option<i32>,
    #[serde(default)]
    pub safety_ratings: Vec<SafetyRating>,
}

/// A set of the feedback metadata the prompt specified in [GenerateContentRequest.content].
///
/// [API Reference](https://ai.google.dev/api/generate-content#PromptFeedback)
#[derive(Debug, Deserialize)]
pub struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRating>,
}

/// Safety rating for a piece of content
///
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of harm categories and the probability of the harm classification is included here.
///
/// [API Reference](https://ai.google.dev/api/generate-content#safetyrating)
#[derive(Debug, Deserialize)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
    #[serde(default)]
    pub blocked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub args: Value,
}

/// The base structured datatype containing multi-part content of a message
///
/// [API Reference](https://ai.google.dev/api/caching#Content)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
    pub role: Role,
    #[serde(default)]
    pub parts: Vec<Part>,
}

/// A datatype containing media that is part of a multi-part Content message
///
/// [API Reference](https://ai.google.dev/api/caching#Part)
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_code: Option<ExecutableCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution_result: Option<CodeExecutionResult>,
    #[serde(rename = "functionCall", skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

impl Part {
    pub fn text(text: &str) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }
}

/// Metadata for a video File
///
/// [API Reference](https://ai.google.dev/api/files#VideoFileMetadata)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    pub start_offset: StartOffset,
    pub end_offset: EndOffset,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndOffset {
    pub seconds: i32,
    pub nanos: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StartOffset {
    pub seconds: i32,
    pub nanos: i32,
}

/// URI based data
///
/// [API Reference](https://ai.google.dev/api/caching#FileData)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}

/// Inline media bytes (stored as a base64 string for some reason) //todo
///
/// [API Reference](https://ai.google.dev/api/caching#Blob)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}

/// Defines the reason why the model stopped generating tokens
///
/// [API Reference](https://ai.google.dev/api/generate-content#FinishReason)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    /// Default value. This value is unused.
    FinishReasonUnspecified,
    /// Natural stop point of the model or provided stop sequence
    Stop,
    /// The maximum number of tokens as specified in the request was reached
    MaxTokens,
    /// The response candidate content was flagged for safety reasons
    Safety,
    /// The response candidate content was flagged for recitation reasons
    Recitation,
    /// The response candidate content was flagged for using an unsupported language
    Language,
    /// Unknown reason
    Other,
    /// Token generation stopped because the content contains forbidden terms
    Blocklist,
    /// Token generation stopped for potentially containing prohibited content
    ProhibitedContent,
    /// Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information (SPII)
    Spii,
    /// The function call generated by the model is invalid
    MalformedFunctionCall,
    /// Token generation stopped because generated images contain safety violations
    ImageSafety,
}

/// The category of a rating
///
/// These categories cover various kinds of harms that developers may wish to adjust
///
/// [API Reference](https://ai.google.dev/api/generate-content#harmcategory)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    /// Category is unspecified
    HarmCategoryUnspecified,
    /// PaLM - Negative or harmful comments targeting identity and/or protected attribute
    HarmCategoryDerogatory,
    /// PaLM - Content that is rude, disrespectful, or profane
    HarmCategoryToxicity,
    /// PaLM - Describes scenarios depicting violence against an individual or group, or general descriptions of gore
    HarmCategoryViolence,
    /// PaLM - Describes scenarios depicting violence against an individual or group, or general descriptions of gore
    HarmCategorySexual,
    /// PaLM - Promotes unchecked medical advice
    HarmCategoryMedical,
    /// PaLM - Dangerous content that promotes, facilitates, or encourages harmful acts
    HarmCategoryDangerous,
    /// Gemini - Harassment content
    HarmCategoryHarassment,
    /// Gemini - Hate speech and content
    HarmCategoryHateSpeech,
    /// Gemini - Sexually explicit content
    HarmCategorySexuallyExplicit,
    /// Gemini - Dangerous content
    HarmCategoryDangerousContent,
    /// Gemini - Content that may be used to harm civic integrity
    HarmCategoryCivicIntegrity,
}

/// Block at and beyond a specified harm probability
///
/// [API Reference](https://ai.google.dev/api/generate-content#HarmBlockThreshold)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    /// Threshold is unspecified
    HarmBlockThresholdUnspecified,
    /// Content with [HarmProbability::Negligible] will be allowed
    BlockLowAndAbove,
    /// Content with [HarmProbability::Negligible] and [HarmProbability::Low] will be allowed
    BlockMedAndAbove,
    /// Content with [HarmProbability::Negligible], [HarmProbability::Low], and
    /// [HarmProbability::Medium] will be allowed
    BlockOnlyHigh,
    /// All content will be allowed
    BlockNone,
    /// Turn off the safety filter
    OFF,
}

/// The probability that a piece of content is harmful
///
/// The classification system gives the probability of the content being unsafe. This does not
/// indicate the severity of harm for a piece of content.
///
/// [API Reference](https://ai.google.dev/api/generate-content#HarmProbability)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    /// Probability is unspecified
    HarmProbabilityUnspecified,
    /// Content has a negligible chance of being unsafe
    Negligible,
    /// Content has a low chance of being unsafe
    Low,
    /// Content has a medium chance of being unsafe
    Medium,
    /// Content has a high chance of being unsafe
    High,
}

/// GoogleSearch tool type.
///
/// Tool to support Google Search in Model. Powered by Google.
///
/// [API Reference](https://ai.google.dev/api/caching#GoogleSearch)
#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleSearchTool {}

/// Tool that executes code generated by the model, and automatically returns the result to the model
///
/// See also [ExecutableCode] and [CodeExecutionResult] which are only generated when using this tool
///
/// [API Reference](https://ai.google.dev/api/caching#CodeExecution)
#[derive(Debug, Deserialize, Serialize)]
pub struct CodeExecutionTool {}

/// Tool details that the model may use to generate response
///
/// A `Tool` is a piece of code that enables the system to interact with external systems to perform
/// an action, or set of actions, outside of knowledge and scope of the model.
///
/// [API Reference](https://ai.google.dev/api/caching#Tool)
#[derive(Debug, Serialize)]
pub struct Tools {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionDeclarations")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "googleSearch")]
    pub google_search: Option<GoogleSearchTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "codeExecution")]
    pub code_execution: Option<CodeExecutionTool>,
}

/// Structured representation of a function declaration
///
/// Defined by the OpenAPI 3.03 specification.
/// `FunctionDeclaration` is a representation of a block of code that can be used in [Tools] by the model and executed by the client.
///
/// [API Reference](https://ai.google.dev/api/caching#FunctionDeclaration)
#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    /// Defines the input parameters the function expects
    ///
    /// Use the [API Reference](https://ai.google.dev/gemini-api/docs/function-calling#function_declarations)
    /// to see how to structure the parameters.
    pub parameters: Value,
}

/// Request to generate content from the model
///
/// [API Reference](https://ai.google.dev/api/generate-content#request-body)
#[derive(Debug, Default, Serialize)]
pub struct GenerateContent {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tools>,
    #[serde(default, rename = "toolConfig")]
    pub tool_config: Option<ToolConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default, rename = "safetySettings")]
    pub safety_settings: Vec<SafetySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "system_instruction")]
    pub system_instruction: Option<SystemInstructionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
}

/// System instructions are used to provide the model with additional context or instructions
///
/// Similar to the [Content] struct, but specifically for system instructions.
#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInstructionContent {
    #[serde(default)]
    pub parts: Vec<SystemInstructionPart>,
}

/// A part of the system instruction content
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInstructionPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Configuration options for model generation and outputs.
///
/// Not all parameters are configurable for every model.
///
/// [API Reference](https://ai.google.dev/api/generate-content#v1beta.GenerationConfig)
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub candidate_count: Option<i32>,
    pub max_output_tokens: Option<i32>,
    pub stop_sequences: Option<Vec<String>>,
    pub response_mime_type: Option<String>,
    pub response_schema: Option<Schema>,
    #[serde(rename = "thinkingConfig", skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

/// Config for thinking features
///
/// [API Reference](https://ai.google.dev/api/generate-content#ThinkingConfig)
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    #[serde(rename = "thinkingBudget", skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<u16>, // 0~24576
    #[serde(rename = "includeThoughts", skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
}

/// Safety setting, affecting the safety-blocking behavior
///
/// Passing a safety setting for a category changes the allowed probability that content is blocked.
///
/// [API Reference](https://ai.google.dev/api/generate-content#safetysetting)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SafetySettings {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

/// The Schema object allows the definition of input and output data types.
///
/// These types can be objects, but also primitives and arrays.
/// Represents a select subset of an [OpenAPI 3.0 schema
/// object](https://spec.openapis.org/oas/v3.0.3#schema).
///
/// [API Reference](https://ai.google.dev/api/caching#Schema)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<String>,
    #[serde(rename = "minItems")]
    pub min_items: Option<String>,
    pub properties: Option<BTreeMap<String, Schema>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "propertyOrdering")]
    pub property_ordering: Option<Vec<String>>,
    pub items: Option<Box<Schema>>,
}

/// The Tool configuration containing parameters for specifying [Tools] use in the request
///
/// [API Reference](https://ai.google.dev/api/caching#ToolConfig)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

/// Configuration for specifying function calling behavior
///
/// [API Reference](https://ai.google.dev/api/caching#FunctionCallingConfig)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

/// Defines the execution behavior for function calling by defining the execution mode
///
/// [API Reference](https://ai.google.dev/api/caching#Mode_1)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {
    /// Unspecified function calling mode. This value should not be used.
    ModeUnspecified,
    /// Default model behavior, model decides to predict either a function call or a natural language response.
    Auto,
    /// Model is constrained to always predicting a function call only. If "allowedFunctionNames" are set, the predicted function call will be limited to any one of "allowedFunctionNames", else the predicted function call will be any one of the provided "functionDeclarations".
    Any,
    /// Model will not predict any function call. Model behavior is same as when not passing any function declarations.
    None,
    /// Model decides to predict either a function call or a natural language response, but will validate function calls with constrained decoding.
    Validated,
}

/// Code generated by the model that is meant to be executed, and the result returned to the model
///
/// Only generated when using the [CodeExecutionTool] tool, in which the code will be automatically executed, and a corresponding [CodeExecutionResult] will also be generated.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExecutableCode {
    #[serde(rename = "language")]
    pub language: ProgrammingLanguage,
    #[serde(rename = "code")]
    pub code: String,
}

/// Supported programming languages for the generated code
///
/// [API Reference](https://ai.google.dev/api/caching#Language)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgrammingLanguage {
    /// Unspecified language. This value should not be used.
    LanguageUnspecified,
    /// Python >= 3.10, with numpy and simpy available.
    Python,
}

/// Enumeration of possible outcomes of the [CodeExecutionTool]
///
/// [API Reference](https://ai.google.dev/api/caching#Outcome)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    /// Unspecified status. This value should not be used.
    OutcomeUnspecified,
    /// Code execution completed successfully.
    OutcomeOk,
    /// Code execution finished but with a failure. stderr should contain the reason.
    OutcomeError,
    /// Code execution ran for too long, and was cancelled. There may or may not be a partial output present.
    OutcomeDeadlineExceeded,
}

/// The result output from a [FunctionCall]
///
/// [API Reference](https://ai.google.dev/api/caching#FunctionResponse)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    //Optional. The function parameters and values in JSON object format.
    pub args: Option<Value>,
}

/// Result of executing the [ExecutableCode]
///
/// [API Reference](https://ai.google.dev/api/caching#CodeExecutionResult)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CodeExecutionResult {
    pub outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Definitions of the types of data that can be used in [Schema]
///
/// Copied from [serde_json](https://docs.rs/serde_json/1.0.140/serde_json/value/enum.Value.html)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Object,
    Array,
    String,
    Integer,
    Number,
    Boolean,
}
