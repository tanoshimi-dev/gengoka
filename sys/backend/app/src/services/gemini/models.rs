use serde::{Deserialize, Serialize};

// ============ Gemini API Request/Response Structures ============

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
    pub generation_config: GenerationConfig,
}

#[derive(Debug, Serialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    pub max_output_tokens: i32,
    pub response_mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<GeminiCandidate>>,
    pub error: Option<GeminiError>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiResponseContent,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseContent {
    pub parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponsePart {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiError {
    pub code: i32,
    pub message: String,
    pub status: String,
}

// ============ Internal DTOs for Challenge Generation ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedChallenge {
    pub title: String,
    pub description: String,
    pub char_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedChallengesResponse {
    pub challenges: Vec<GeneratedChallenge>,
}

impl GeminiRequest {
    pub fn new(prompt: String, max_tokens: i32, temperature: f32) -> Self {
        Self {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text: prompt }],
            }],
            generation_config: GenerationConfig {
                temperature,
                max_output_tokens: max_tokens,
                response_mime_type: "application/json".to_string(),
            },
        }
    }
}

impl GeminiResponse {
    pub fn get_text(&self) -> Option<String> {
        self.candidates
            .as_ref()?
            .first()?
            .content
            .parts
            .first()
            .map(|p| p.text.clone())
    }
}
