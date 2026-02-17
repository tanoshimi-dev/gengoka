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

const LINE_PROFILE_URL: &str = "https://api.line.me/v2/profile";
const LINE_VERIFY_URL: &str = "https://api.line.me/oauth2/v2.1/verify";

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
        .get(LINE_VERIFY_URL)
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
        .get(LINE_PROFILE_URL)
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
