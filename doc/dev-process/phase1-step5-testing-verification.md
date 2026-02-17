# Phase 1 Step 5: テスト・検証 (Testing & Verification)

## Overview

Added backend unit tests with HTTP mocking for all social authentication providers (Google, Apple, LINE) and utility functions. Total: **21 tests** across 4 files.

## Changes Made

### 1. Dev-Dependencies (`Cargo.toml`)

Added test-only dependencies:
- `wiremock = "0.6"` — HTTP mock server for simulating external API responses
- `rsa = "0.9"` — RSA key pair generation for creating test JWTs
- `base64 = "0.22"` — Base64url encoding of RSA key components for JWKS mocks

### 2. Configurable External URLs (`config/mod.rs`)

Added 4 URL fields to `SocialAuthConfig`:
- `google_jwks_url` — Google's JWKS endpoint (default: `https://www.googleapis.com/oauth2/v3/certs`)
- `apple_jwks_url` — Apple's JWKS endpoint (default: `https://appleid.apple.com/auth/keys`)
- `line_verify_url` — LINE token verify endpoint (default: `https://api.line.me/oauth2/v2.1/verify`)
- `line_profile_url` — LINE profile endpoint (default: `https://api.line.me/v2/profile`)

Production behavior unchanged — URLs default to real endpoints. In tests, they point to `wiremock::MockServer`.

### 3. Google Auth Tests (`services/social_auth/google.rs`) — 5 tests

| Test | Verifies |
|------|----------|
| `test_verify_google_token_valid` | Valid JWT → correct SocialUserInfo extraction |
| `test_verify_google_token_invalid_audience` | Wrong `aud` → rejection |
| `test_verify_google_token_expired` | Past `exp` → rejection |
| `test_verify_google_token_wrong_kid` | Mismatched `kid` → "No matching Google key found" |
| `test_verify_google_token_malformed` | Garbage input → "Invalid token header" |

### 4. Apple Auth Tests (`services/social_auth/apple.rs`) — 5 tests

| Test | Verifies |
|------|----------|
| `test_verify_apple_token_valid` | Valid JWT → success with email-prefix name |
| `test_verify_apple_token_no_email` | `email=None` → `name=None`, `email=None` |
| `test_verify_apple_token_invalid_issuer` | Wrong issuer → rejection |
| `test_verify_apple_token_expired` | Past `exp` → rejection |
| `test_verify_apple_token_wrong_kid` | Mismatched `kid` → "No matching Apple key found" |

### 5. LINE Auth Tests (`services/social_auth/line.rs`) — 5 tests

| Test | Verifies |
|------|----------|
| `test_verify_line_token_valid` | Mock verify + profile → correct extraction |
| `test_verify_line_token_channel_mismatch` | Wrong `client_id` → "channel ID mismatch" |
| `test_verify_line_token_expired` | `expires_in=0` → "LINE token expired" |
| `test_verify_line_token_no_picture` | `pictureUrl` absent → `avatar=None` |
| `test_verify_line_token_verify_api_error` | HTTP 400 from verify → parse error |

### 6. Utility Function Tests (`utils/mod.rs`) — 6 tests

| Test | Verifies |
|------|----------|
| `test_hash_and_verify_password` | Argon2 hash + verify correct/incorrect passwords |
| `test_generate_access_token_and_decode` | Generate JWT → decode → verify `sub` matches |
| `test_decode_expired_token` | Token with past `exp` → error |
| `test_refresh_token_uniqueness` | Two generated tokens differ |
| `test_hash_refresh_token_deterministic` | Same input → same SHA-256 hash |
| `test_normalize_pagination` | Default, explicit, clamped, and edge-case inputs |

## How to Run

The `gengoka-backend` container is a runtime-only image (no Rust toolchain). Use a separate `rust` image to run tests:

```bash
cd sys/backend/app
docker run --rm -v $(pwd):/app -w /app rust:latest cargo test
```

All tests are fully self-contained — no external network, database, or environment variables required.

## Files Modified

- `sys/backend/app/Cargo.toml` — added `[dev-dependencies]`
- `sys/backend/app/src/config/mod.rs` — added URL fields to `SocialAuthConfig`
- `sys/backend/app/src/services/social_auth/google.rs` — configurable URL + 5 tests
- `sys/backend/app/src/services/social_auth/apple.rs` — configurable URL + 5 tests
- `sys/backend/app/src/services/social_auth/line.rs` — configurable URLs + 5 tests
- `sys/backend/app/src/utils/mod.rs` — 6 tests
