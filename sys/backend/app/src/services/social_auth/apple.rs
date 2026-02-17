use crate::config::SocialAuthConfig;
use crate::models::SocialUserInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppleJwks {
    keys: Vec<AppleJwk>,
}

#[derive(Debug, Deserialize)]
struct AppleJwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct AppleClaims {
    sub: String,
    email: Option<String>,
    aud: String,
}

const APPLE_JWKS_URL: &str = "https://appleid.apple.com/auth/keys";

/// Verify an Apple ID token and extract user info.
///
/// Flow:
/// 1. Fetch Apple's public JWK set
/// 2. Decode the JWT header to find the key ID (kid)
/// 3. Find the matching public key and verify the signature
/// 4. Validate audience (client_id) and issuer
/// 5. Extract user info from claims
///
/// Note: Apple only provides the user's name on the very first sign-in.
/// On subsequent sign-ins, only `sub` and `email` are available in the token.
pub async fn verify_apple_token(
    id_token: &str,
    config: &SocialAuthConfig,
) -> Result<SocialUserInfo, String> {
    // Fetch Apple's public keys
    let client = reqwest::Client::new();
    let jwks: AppleJwks = client
        .get(APPLE_JWKS_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Apple JWKS: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse Apple JWKS: {}", e))?;

    // Decode JWT header to get kid
    let header = jsonwebtoken::decode_header(id_token)
        .map_err(|e| format!("Invalid token header: {}", e))?;

    let kid = header.kid.ok_or("Token missing kid")?;

    // Find matching key
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or("No matching Apple key found")?;

    // Build decoding key from RSA components
    let decoding_key =
        jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| format!("Invalid RSA key: {}", e))?;

    // Validate and decode
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[&config.apple_client_id]);
    validation.set_issuer(&["https://appleid.apple.com"]);

    let token_data = jsonwebtoken::decode::<AppleClaims>(id_token, &decoding_key, &validation)
        .map_err(|e| format!("Token verification failed: {}", e))?;

    let claims = token_data.claims;

    // Apple only provides name on first sign-in; extract email-based name as fallback
    let name = claims
        .email
        .as_ref()
        .and_then(|e| e.split('@').next())
        .map(|s| s.to_string());

    Ok(SocialUserInfo {
        provider: "apple".to_string(),
        provider_user_id: claims.sub,
        email: claims.email,
        name,
        avatar: None, // Apple doesn't provide avatar
    })
}
