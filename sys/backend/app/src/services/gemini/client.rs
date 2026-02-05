use anyhow::{anyhow, Context, Result};
use backoff::{future::retry, ExponentialBackoff};
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, warn};

use super::models::{GeneratedChallengesResponse, GeminiRequest, GeminiResponse};
use crate::config::GeminiConfig;

pub struct GeminiClient {
    client: Client,
    config: GeminiConfig,
}

impl GeminiClient {
    pub fn new(config: GeminiConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, config })
    }

    pub async fn generate_challenges(
        &self,
        prompt: String,
    ) -> Result<GeneratedChallengesResponse> {
        let request = GeminiRequest::new(
            prompt,
            self.config.max_tokens,
            self.config.temperature,
        );

        let response_text = self.call_api_with_retry(&request).await?;
        let json_text = extract_json_from_response(&response_text)?;

        let challenges: GeneratedChallengesResponse = serde_json::from_str(&json_text)
            .with_context(|| format!("Failed to parse challenges JSON: {}", json_text))?;

        Ok(challenges)
    }

    async fn call_api_with_retry(&self, request: &GeminiRequest) -> Result<String> {
        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(Duration::from_secs(120)),
            initial_interval: Duration::from_secs(1),
            max_interval: Duration::from_secs(30),
            multiplier: 2.0,
            ..Default::default()
        };

        let result = retry(backoff, || async {
            match self.call_api(request).await {
                Ok(text) => Ok(text),
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.contains("429") || error_str.contains("rate limit") {
                        warn!("Rate limited, will retry: {}", error_str);
                        Err(backoff::Error::transient(e))
                    } else if error_str.contains("500") || error_str.contains("503") {
                        warn!("Server error, will retry: {}", error_str);
                        Err(backoff::Error::transient(e))
                    } else {
                        Err(backoff::Error::permanent(e))
                    }
                }
            }
        })
        .await?;

        Ok(result)
    }

    async fn call_api(&self, request: &GeminiRequest) -> Result<String> {
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.config.base_url, self.config.model, self.config.api_key
        );

        debug!("Calling Gemini API: {}", self.config.model);

        let response = self
            .client
            .post(&url)
            .json(request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Gemini API error ({}): {}",
                status.as_u16(),
                error_body
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        if let Some(error) = gemini_response.error {
            return Err(anyhow!(
                "Gemini API error ({}): {}",
                error.code,
                error.message
            ));
        }

        gemini_response
            .get_text()
            .ok_or_else(|| anyhow!("No text in Gemini API response"))
    }
}

fn extract_json_from_response(text: &str) -> Result<String> {
    let trimmed = text.trim();

    // If it starts with [ or {, it's already JSON
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        return Ok(trimmed.to_string());
    }

    // Try to extract JSON from markdown code blocks
    if let Some(start) = trimmed.find("```json") {
        let after_marker = &trimmed[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    // Try generic code blocks
    if let Some(start) = trimmed.find("```") {
        let after_marker = &trimmed[start + 3..];
        // Skip language identifier if present
        let json_start = after_marker.find('\n').unwrap_or(0);
        let after_newline = &after_marker[json_start..];
        if let Some(end) = after_newline.find("```") {
            return Ok(after_newline[..end].trim().to_string());
        }
    }

    // Return as-is and let JSON parsing handle errors
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_direct() {
        let input = r#"{"challenges": []}"#;
        let result = extract_json_from_response(input).unwrap();
        assert_eq!(result, r#"{"challenges": []}"#);
    }

    #[test]
    fn test_extract_json_from_markdown() {
        let input = r#"
Here are the challenges:

```json
{"challenges": [{"title": "Test"}]}
```
"#;
        let result = extract_json_from_response(input).unwrap();
        assert_eq!(result, r#"{"challenges": [{"title": "Test"}]}"#);
    }

    #[test]
    fn test_extract_json_array() {
        let input = r#"[{"title": "Test"}]"#;
        let result = extract_json_from_response(input).unwrap();
        assert_eq!(result, r#"[{"title": "Test"}]"#);
    }
}
