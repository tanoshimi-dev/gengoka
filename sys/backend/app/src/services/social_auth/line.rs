use crate::config::SocialAuthConfig;
use crate::models::SocialUserInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LineProfile {
    user_id: String,
    display_name: String,
    picture_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LineVerifyResponse {
    client_id: String,
    expires_in: i64,
}

/// Verify a LINE access token and extract user info.
///
/// Flow:
/// 1. Verify the access token is valid and belongs to our channel
/// 2. Fetch the user's LINE profile using the access token
/// 3. Extract user info from the profile
///
/// Note: LINE doesn't expose email through the profile API without
/// special permission. Email is only available via the ID token (OpenID Connect).
pub async fn verify_line_token(
    access_token: &str,
    config: &SocialAuthConfig,
) -> Result<SocialUserInfo, String> {
    let client = reqwest::Client::new();

    // Step 1: Verify the access token
    let verify_resp: LineVerifyResponse = client
        .get(&config.line_verify_url)
        .query(&[("access_token", access_token)])
        .send()
        .await
        .map_err(|e| format!("Failed to verify LINE token: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse LINE verify response: {}", e))?;

    // Validate channel ID
    if verify_resp.client_id != config.line_channel_id {
        return Err("LINE token channel ID mismatch".to_string());
    }

    if verify_resp.expires_in <= 0 {
        return Err("LINE token expired".to_string());
    }

    // Step 2: Fetch user profile
    let profile: LineProfile = client
        .get(&config.line_profile_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch LINE profile: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse LINE profile: {}", e))?;

    Ok(SocialUserInfo {
        provider: "line".to_string(),
        provider_user_id: profile.user_id,
        email: None, // LINE profile API doesn't provide email
        name: Some(profile.display_name),
        avatar: profile.picture_url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(verify_url: &str, profile_url: &str) -> SocialAuthConfig {
        SocialAuthConfig {
            google_client_id_web: String::new(),
            google_client_id_ios: String::new(),
            google_client_id_android: String::new(),
            apple_client_id: String::new(),
            apple_client_id_web: String::new(),
            apple_team_id: String::new(),
            line_channel_id: "test-channel-id".to_string(),
            line_channel_secret: "test-channel-secret".to_string(),
            google_jwks_url: String::new(),
            apple_jwks_url: String::new(),
            line_verify_url: verify_url.to_string(),
            line_profile_url: profile_url.to_string(),
        }
    }

    #[tokio::test]
    async fn test_verify_line_token_valid() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "client_id": "test-channel-id",
                "expires_in": 3600
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/profile"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "userId": "line-user-123",
                "displayName": "LINE User",
                "pictureUrl": "https://profile.line-scdn.net/abc"
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(
            &format!("{}/verify", mock_server.uri()),
            &format!("{}/profile", mock_server.uri()),
        );

        let result = verify_line_token("valid-access-token", &config).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.provider, "line");
        assert_eq!(info.provider_user_id, "line-user-123");
        assert_eq!(info.name, Some("LINE User".to_string()));
        assert_eq!(
            info.avatar,
            Some("https://profile.line-scdn.net/abc".to_string())
        );
        assert_eq!(info.email, None);
    }

    #[tokio::test]
    async fn test_verify_line_token_channel_mismatch() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "client_id": "wrong-channel-id",
                "expires_in": 3600
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(
            &format!("{}/verify", mock_server.uri()),
            &format!("{}/profile", mock_server.uri()),
        );

        let result = verify_line_token("some-token", &config).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "LINE token channel ID mismatch");
    }

    #[tokio::test]
    async fn test_verify_line_token_expired() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "client_id": "test-channel-id",
                "expires_in": 0
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(
            &format!("{}/verify", mock_server.uri()),
            &format!("{}/profile", mock_server.uri()),
        );

        let result = verify_line_token("some-token", &config).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "LINE token expired");
    }

    #[tokio::test]
    async fn test_verify_line_token_no_picture() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "client_id": "test-channel-id",
                "expires_in": 3600
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/profile"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "userId": "line-user-456",
                "displayName": "No Avatar User"
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(
            &format!("{}/verify", mock_server.uri()),
            &format!("{}/profile", mock_server.uri()),
        );

        let result = verify_line_token("valid-token", &config).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.provider_user_id, "line-user-456");
        assert_eq!(info.avatar, None);
    }

    #[tokio::test]
    async fn test_verify_line_token_verify_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/verify"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({"error": "invalid_request"})),
            )
            .mount(&mock_server)
            .await;

        let config = test_config(
            &format!("{}/verify", mock_server.uri()),
            &format!("{}/profile", mock_server.uri()),
        );

        let result = verify_line_token("bad-token", &config).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to parse LINE verify response"));
    }
}
