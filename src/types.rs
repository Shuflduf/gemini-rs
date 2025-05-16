use std::{collections::BTreeMap, default, hash::Hash};

use serde::{Deserialize, Serialize};
use serde_json::Map;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    InvalidArgument,
    FailedPrecondition,
    PermissionDenied,
    NotFound,
    ResourceExhausted,
    Internal,
    Unavailable,
    DeadlineExceeded,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Models {
    pub models: Vec<Model>,
    pub next_page_token: Option<String>,
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: u64,
    pub candidates_token_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: Option<FinishReason>,
    pub index: Option<i32>,
    #[serde(default)]
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Deserialize)]
pub struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Deserialize)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
    #[serde(default)]
    pub blocked: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
    pub role: Role,
    #[serde(default)]
    pub parts: Vec<Part>,
}

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
    pub function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution_result: Option<CodeExecutionResult>,
    
}

impl Part {
    pub fn text(text: &str) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }
}



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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Language,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
    MalformedFunctionCall,
    ImageSafety,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    HarmCategoryUnspecified,
    HarmCategoryDerogatory,
    HarmCategoryToxicity,
    HarmCategoryViolence,
    HarmCategorySexual,
    HarmCategoryMedical,
    HarmCategoryDangerous,
    HarmCategoryHarassment,
    HarmCategoryHateSpeech,
    HarmCategorySexuallyExplicit,
    HarmCategoryDangerousContent,
    HarmCategoryCivicIntegrity,
}


// https://ai.google.dev/api/generate-content?hl=en#safetysetting
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    OFF,
    BlockNone,
    BlockLowAndAbove,
    BlockMedAndAbove,
    //BlockHighAndAbove, // It seems like deprecated
    HarmBlockThresholdUnspecified,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    HarmProbabilityUnspecified,
    Negligible,
    Low,
    Medium,
    High,
}

// when use this feature, 
// you can't use function_declarations by google's policy
#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleSearchTool{}
#[derive(Debug, Deserialize, Serialize)]
pub struct CodeExecutionTool {}

//https://ai.google.dev/gemini-api/docs/function-calling?hl=en&example=weather#multi-tool_use_combine_native_tools_with_function_calling
#[derive(Debug, Serialize)]
pub struct Tools {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionDeclarations")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "googleSearch")]
    pub google_search:Option<GoogleSearchTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "codeExecution")]
    pub code_execution:Option<CodeExecutionTool>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    // https://ai.google.dev/api/caching?hl=en#FunctionDeclaration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Schema>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    sche_type: String,
    #[serde(rename = "properties")]
    pub properties:Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize)]
pub struct GenerateContent {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tools>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default, rename = "safetySettings")]
    pub safety_settings: Vec<SafetySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "system_instruction")]
    pub system_instruction: Option<SystemInstructionContent>,
    #[serde(default, rename = "toolConfig")]
    pub tool_config: Option<ToolConfig>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInstructionContent {
    #[serde(default)]
    pub parts: Vec<SystemInstructionPart>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInstructionPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

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
    #[serde(rename="thinkingConfig",skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

#[derive(Debug, Default,Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    #[serde(rename="thinkingBudget",skip_serializing_if = "Option::is_none")]
    pub thinking_budget : Option<u16>, // 0~24576
    #[serde(rename="includeThoughts",skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SafetySettings {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

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

//https://ai.google.dev/api/caching?hl=en#ToolConfig
#[derive(Debug, Deserialize, Serialize)]
pub struct ToolConfig {
    #[serde(rename = "functionCallingConfig",skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig{
    #[serde(rename = "mode",skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,
    #[serde(rename = "allowedFunctionNames",skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {
    Auto,
    Any,
    None,
    Validated,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExecutableCode {
    #[serde(rename = "language")]
    pub language: ProgrammingLanguage,
    #[serde(rename = "code")]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgrammingLanguage {
    LanguageUnspecified,
    Python,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    OutcomeUnspecified,
    OutcomeOk,
    OutcomeError,
    OutcomeDeadlineExceeded,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionCall {
    #[serde(rename = "id",skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    pub name:String,
    #[serde(rename = "args")]
    pub args: serde_json::Value,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionResponse{
    #[serde(rename = "id",skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    pub name:String,
    #[serde(rename = "args")]
    pub args: serde_json::Value,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CodeExecutionResult {
    #[serde(rename = "outcome")]
    pub outcome:Outcome,
    #[serde(rename = "output",skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

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
