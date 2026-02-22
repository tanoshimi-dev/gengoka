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
        .get(&config.google_jwks_url)
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

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_KID: &str = "test-google-key-1";

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
            google_client_id_web: "test-web-client-id".to_string(),
            google_client_id_ios: "test-ios-client-id".to_string(),
            google_client_id_android: "test-android-client-id".to_string(),
            apple_client_id: String::new(),
            apple_client_id_web: String::new(),
            apple_team_id: String::new(),
            line_channel_id: String::new(),
            line_channel_secret: String::new(),
            google_jwks_url: jwks_url.to_string(),
            apple_jwks_url: String::new(),
            line_verify_url: String::new(),
            line_profile_url: String::new(),
        }
    }

    #[derive(serde::Serialize)]
    struct TestGoogleClaims {
        sub: String,
        email: Option<String>,
        email_verified: Option<bool>,
        name: Option<String>,
        picture: Option<String>,
        aud: String,
        iss: String,
        exp: usize,
        iat: usize,
    }

    fn valid_claims(aud: &str) -> TestGoogleClaims {
        let now = chrono::Utc::now();
        TestGoogleClaims {
            sub: "google-user-123".to_string(),
            email: Some("test@gmail.com".to_string()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            picture: Some("https://example.com/photo.jpg".to_string()),
            aud: aud.to_string(),
            iss: "accounts.google.com".to_string(),
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }

    #[tokio::test]
    async fn test_verify_google_token_valid() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let claims = valid_claims("test-web-client-id");
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_google_token(&token, &config).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.provider, "google");
        assert_eq!(info.provider_user_id, "google-user-123");
        assert_eq!(info.email, Some("test@gmail.com".to_string()));
        assert_eq!(info.name, Some("Test User".to_string()));
        assert_eq!(
            info.avatar,
            Some("https://example.com/photo.jpg".to_string())
        );
    }

    #[tokio::test]
    async fn test_verify_google_token_invalid_audience() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let claims = valid_claims("wrong-audience");
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_google_token(&token, &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token verification failed"));
    }

    #[tokio::test]
    async fn test_verify_google_token_expired() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let now = chrono::Utc::now();
        let claims = TestGoogleClaims {
            exp: (now - chrono::Duration::hours(1)).timestamp() as usize,
            iat: (now - chrono::Duration::hours(2)).timestamp() as usize,
            ..valid_claims("test-web-client-id")
        };
        let token = create_test_jwt(&claims, TEST_KID, &pem);

        let result = verify_google_token(&token, &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token verification failed"));
    }

    #[tokio::test]
    async fn test_verify_google_token_wrong_kid() {
        let (pem, jwks) = generate_test_keys();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));
        let claims = valid_claims("test-web-client-id");
        let token = create_test_jwt(&claims, "wrong-kid", &pem);

        let result = verify_google_token(&token, &config).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No matching Google key found");
    }

    #[tokio::test]
    async fn test_verify_google_token_malformed() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/jwks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&serde_json::json!({"keys": []})))
            .mount(&mock_server)
            .await;

        let config = test_config(&format!("{}/jwks", mock_server.uri()));

        let result = verify_google_token("not-a-valid-jwt", &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid token header"));
    }
}
