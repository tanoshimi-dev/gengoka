use crate::config::SocialAuthConfig;
use crate::models::SocialUserInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoogleJwks {
    keys: Vec<GoogleJwk>,
}

#[derive(Debug, Deserialize)]
struct GoogleJwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct GoogleClaims {
    sub: String,
    email: Option<String>,
    email_verified: Option<bool>,
    name: Option<String>,
    picture: Option<String>,
    aud: String,
}

const GOOGLE_JWKS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

/// Verify a Google ID token and extract user info.
///
/// Flow:
/// 1. Fetch Google's public JWK set
/// 2. Decode the JWT header to find the key ID (kid)
/// 3. Find the matching public key and verify the signature
/// 4. Validate the audience matches our client ID
/// 5. Extract user info from claims
pub async fn verify_google_token(
    id_token: &str,
    config: &SocialAuthConfig,
) -> Result<SocialUserInfo, String> {
    // Fetch Google's public keys
    let client = reqwest::Client::new();
    let jwks: GoogleJwks = client
        .get(GOOGLE_JWKS_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Google JWKS: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse Google JWKS: {}", e))?;

    // Decode JWT header to get kid
    let header = jsonwebtoken::decode_header(id_token)
        .map_err(|e| format!("Invalid token header: {}", e))?;

    let kid = header.kid.ok_or("Token missing kid")?;

    // Find matching key
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or("No matching Google key found")?;

    // Build decoding key from RSA components
    let decoding_key =
        jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| format!("Invalid RSA key: {}", e))?;

    // Validate and decode
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[
        &config.google_client_id_web,
        &config.google_client_id_ios,
        &config.google_client_id_android,
    ]);
    validation.set_issuer(&["accounts.google.com", "https://accounts.google.com"]);

    let token_data = jsonwebtoken::decode::<GoogleClaims>(id_token, &decoding_key, &validation)
        .map_err(|e| format!("Token verification failed: {}", e))?;

    let claims = token_data.claims;

    Ok(SocialUserInfo {
        provider: "google".to_string(),
        provider_user_id: claims.sub,
        email: claims.email,
        name: claims.name,
        avatar: claims.picture,
    })
}
