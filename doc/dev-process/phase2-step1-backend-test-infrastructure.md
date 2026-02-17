# Phase 2 Step 1: Backend テスト基盤構築

## Summary

Set up the foundation for backend integration testing: a dedicated test database (`gengoka_test`), test helper utilities, and 10 auth endpoint integration tests using `actix-web::test` with a real PostgreSQL database.

## What Was Done

### 1. Test Database Setup
- Created `sys/backend/init-test-db.sh` — Postgres init script that creates `gengoka_test` database on first container startup
- Updated `sys/backend/docker-compose.dev.yml` — Mounted init script into postgres container's entrypoint directory

### 2. Library Crate (`src/lib.rs`)
- Created `src/lib.rs` to expose all modules as a library crate so integration tests can import them
- Updated `src/main.rs` to use `gengoka_backend::*` imports instead of local `mod` declarations

### 3. Dev Dependencies
- Added `serial_test = "3"` to `Cargo.toml` dev-dependencies for serial test execution

### 4. Test Helpers (`tests/common/mod.rs`)
- `test_config()` — Returns a `Config` with test DB credentials and test JWT secret
- `setup_test_db()` — Connects to `gengoka_test`, runs migrations, returns pool
- `clean_db()` — TRUNCATE all tables with CASCADE for clean state between tests
- `create_test_app()` — Builds actix-web test service with routes + pool + config
- `register_test_user()` — POST /auth/register helper, returns (status, json)
- `login_test_user()` — POST /auth/login helper, returns (status, json)

### 5. Auth Integration Tests (`tests/auth_test.rs`)
10 tests covering all auth endpoints:

| Test | Endpoint | Verifies |
|------|----------|----------|
| `test_register_success` | POST /auth/register | 201, returns tokens + user |
| `test_register_duplicate_email` | POST /auth/register | 409 conflict |
| `test_register_invalid_email` | POST /auth/register | 400 validation error |
| `test_register_short_password` | POST /auth/register | 400 validation error |
| `test_login_success` | POST /auth/login | 200, returns tokens |
| `test_login_wrong_password` | POST /auth/login | 401 |
| `test_login_nonexistent_user` | POST /auth/login | 401 |
| `test_refresh_token_success` | POST /auth/refresh | 200, token rotation |
| `test_refresh_token_invalid` | POST /auth/refresh | 401 |
| `test_full_auth_flow` | All auth endpoints | Register → Login → Refresh → Logout → verify old token invalid |

All tests use `#[serial]` to prevent parallel DB access conflicts.

## Files Changed

| File | Action |
|------|--------|
| `sys/backend/init-test-db.sh` | Created |
| `sys/backend/docker-compose.dev.yml` | Modified (added init script volume) |
| `sys/backend/app/Cargo.toml` | Modified (added serial_test) |
| `sys/backend/app/src/lib.rs` | Created |
| `sys/backend/app/src/main.rs` | Modified (use lib crate imports) |
| `sys/backend/app/tests/common/mod.rs` | Created |
| `sys/backend/app/tests/auth_test.rs` | Created |

## How to Run

Recreate postgres container (once, to trigger init script):

```bash
cd sys/backend
docker compose -f docker-compose.dev.yml down -v
docker compose -f docker-compose.dev.yml up -d
```

Run all tests:

```bash
cd sys/backend/app
docker run --rm --network backend_gengoka-network \
  -v $(pwd):/app -w /app \
  rust:latest cargo test
```

Expected: 10 integration tests + 26 existing unit tests = 36 total.
