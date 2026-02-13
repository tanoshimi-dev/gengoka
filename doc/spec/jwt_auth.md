JWT Authentication for Multi-Platform (iOS / Android / Web)

1. Cargo.toml - Added dependencies

  - jsonwebtoken = "9" (JWT encode/decode)
  - sha2 = "0.10" (refresh token hashing)

2. src/config/mod.rs - Added AuthConfig

  - jwt_secret (env: JWT_SECRET, dev default provided)
  - access_token_ttl_minutes (env: ACCESS_TOKEN_TTL_MINUTES, default 30)
  - refresh_token_ttl_days (env: REFRESH_TOKEN_TTL_DAYS, default 90)

3. src/db/mod.rs - Migrations

  - password_hash column added to users table (nullable, idempotent)
  - refresh_tokens table: user_id, token_hash, device_info, expires_at, created_at
  - Indexes on refresh_tokens(user_id) and refresh_tokens(token_hash)

4. src/models/mod.rs - Auth types

  - SignupRequest (email validated, password min 8, name, optional device_info)
  - LoginRequest (email, password, optional device_info)
  - RefreshRequest (refresh_token)
  - LogoutRequest (optional refresh_token)
  - AuthTokens (access_token, refresh_token, expires_in, user: UserSummary)
  - Claims (sub, exp, iat)

5. src/utils/mod.rs - Shared helpers

  - hash_password / verify_password: Argon2 (same pattern as admin auth)
  - generate_access_token / decode_access_token: JWT with HS256
  - generate_refresh_token: UUID v4
  - hash_refresh_token: SHA-256 hex
  - get_user_id updated: tries Authorization: Bearer <jwt> first, falls back to X-User-ID header (backward compatible)

6. src/handlers/auth.rs - 4 endpoints (NEW file)

  ┌────────────────────────┬──────────────────────────────────────────────────────────────┐
  │       Endpoint         │                        Behavior                              │
  ├────────────────────────┼──────────────────────────────────────────────────────────────┤
  │ POST /auth/register    │ Validate -> check email unique -> argon2 hash -> insert user │
  │                        │ -> generate token pair -> store refresh hash -> return tokens │
  ├────────────────────────┼──────────────────────────────────────────────────────────────┤
  │ POST /auth/login       │ Find user by email -> verify password -> generate tokens     │
  │                        │ -> store refresh hash -> return tokens                       │
  ├────────────────────────┼──────────────────────────────────────────────────────────────┤
  │ POST /auth/refresh     │ SHA-256 hash incoming token -> lookup in DB -> check expiry  │
  │                        │ -> delete old token -> issue new pair (rotation) -> return   │
  ├────────────────────────┼──────────────────────────────────────────────────────────────┤
  │ POST /auth/logout      │ If refresh_token provided: delete that token                 │
  │                        │ If omitted: delete all tokens for user (logout everywhere)   │
  └────────────────────────┴──────────────────────────────────────────────────────────────┘

7. src/handlers/mod.rs / src/routes/mod.rs - Wiring

  - pub mod auth + pub use auth::* in handlers
  - 4 routes added under /api/v1/auth/*

Design decisions:
  - Existing handlers unchanged: get_user_id checks JWT first, falls back to X-User-ID
  - Refresh token rotation: old token deleted on each refresh call
  - Refresh tokens stored as SHA-256 hash (never plaintext)
  - password_hash nullable on users: existing users without passwords unaffected
  - No LoginRequest naming conflict: user auth vs admin auth in separate modules

Note: Rust/Cargo is not installed on this machine, so the build could not be verified. Please run cargo build to confirm compilation.
