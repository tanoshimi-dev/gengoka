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
        .get(&config.apple_jwks_url)
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
    let mut audiences: Vec<&str> = vec![&config.apple_client_id];
    if !config.apple_client_id_web.is_empty() {
        audiences.push(&config.apple_client_id_web);
    }
    validation.set_audience(&audiences);
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

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_KID: &str = "test-apple-key-1";

    fn generate_test_keys() -> (Vec<u8>, serde_json::Value) {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use rsa::pkcs8::EncodePrivateKey;
        use rsa::traits::PublicKeyParts;
        use rsa::RsaPrivateKey;

        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();

        let pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap();

        let n = URL_SAFE_NO_PAD.encode(private_key.n().to_bytes_be());
        let e = URL_SAFE_NO_PAD.encode(private_key.e().to_bytes_be());

        let jwks = serde_json::json!({
            "keys": [{
                "kid": TEST_KID,
                "kty": "RSA",
                "n": n,
                "e": e,
                "alg": "RS256",
                "use": "sig"
            }]
        });

        (pem.as_bytes().to_vec(), jwks)
    }

    fn create_test_jwt(claims: &impl serde::Serialize, kid: &str, pem: &[u8]) -> String {
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
        header.kid = Some(kid.to_string());
        jsonwebtoken::encode(
            &header,
            claims,
            &jsonwebtoken::EncodingKey::from_rsa_pem(pem).unwrap(),
        )
        .unwrap()
    }

    fn test_config(jwks_url: &str) -> SocialAuthConfig {
        SocialAuthConfig {
            google_client_id_web: String::new(),
            google_client_id_ios: String::new(),
            google_client_id_android: String::new(),
            apple_client_id: "app.gengoka".to_string(),
            apple_client_id_web: String::new(),
            apple_team_id: "test-team-id".to_string(),
            line_channel_id: String::new(),
            line_channel_secret: String::new(),
            google_jwks_url: String::new(),
            apple_jwks_url: jwks_url.to_string(),
            line_verify_url: String::new(),
            line_profile_url: String::new(),
        }
    }

    #[derive(serde::Serialize)]
    struct TestAppleClaims {
        sub: String,
        email: Option<String>,
        aud: String,
        iss: String,
        exp: usize,
        iat: usize,
    }

    fn valid_claims() -> TestAppleClaims {
        let now = chrono::Utc::now();
        TestAppleClaims {
            sub: "apple-user-001.abc.def".to_string(),
            email: Some("user@privaterelay.appleid.com".to_string()),
            aud: "app.gengoka".to_string(),
            iss: "https://appleid.apple.com".to_string(),
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }

    #[tokio::test]
    async fn test_verify_apple_token_valid() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let claims = valid_claims();
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_apple_token(&token, &config).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.provider, "apple");
        assert_eq!(info.provider_user_id, "apple-user-001.abc.def");
        assert_eq!(
            info.email,
            Some("user@privaterelay.appleid.com".to_string())
        );
        assert_eq!(info.name, Some("user".to_string())); // email prefix
        assert_eq!(info.avatar, None);
    }

    #[tokio::test]
    async fn test_verify_apple_token_no_email() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let mut claims = valid_claims();
        claims.email = None;
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_apple_token(&token, &config).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.name, None);
        assert_eq!(info.email, None);
    }

    #[tokio::test]
    async fn test_verify_apple_token_invalid_issuer() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let mut claims = valid_claims();
        claims.iss = "https://evil.example.com".to_string();
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_apple_token(&token, &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token verification failed"));
    }

    #[tokio::test]
    async fn test_verify_apple_token_expired() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let now = chrono::Utc::now();
        let mut claims = valid_claims();
        claims.exp = (now - chrono::Duration::hours(1)).timestamp() as usize;
        claims.iat = (now - chrono::Duration::hours(2)).timestamp() as usize;
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_apple_token(&token, &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token verification failed"));
    }

    #[tokio::test]
    async fn test_verify_apple_token_wrong_kid() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let claims = valid_claims();
        let token = create_test_jwt(&claims, "wrong-kid", &pem);

        let result = verify_apple_token(&token, &config).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No matching Apple key found");
    }
}
