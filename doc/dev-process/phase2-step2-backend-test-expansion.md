# Phase 2 Step 2: Backend Test Expansion

## Summary

Expanded integration test coverage from 10 (auth only) to 55 tests across all handler groups. Combined with 26 existing unit tests, the backend now has 81 total tests.

## What Was Done

### New Test Helpers (`tests/common/mod.rs`)

| Helper | Purpose |
|--------|---------|
| `register_and_login()` | Register + extract `(user_id, access_token)` |
| `create_test_category()` | Direct DB insert into `categories` |
| `create_test_challenge()` | Direct DB insert into `challenges` |
| `create_test_answer()` | POST `/challenges/{id}/answers` via API |
| `insert_social_account()` | Direct DB insert into `user_social_accounts` |

### New Test Files

| File | Tests | Coverage |
|------|-------|----------|
| `tests/user_test.rs` | 14 | GET /users/me, GET /users/{id}, PUT /users/{id}, GET /users/me/stats, follow/unfollow |
| `tests/challenge_test.rs` | 10 | GET /categories, GET /categories/{id}, POST /challenges, GET /challenges, GET /challenges/{id}, GET /challenges/daily |
| `tests/answer_test.rs` | 17 | POST answers, GET answers, PUT/DELETE answers, likes, comments |
| `tests/feed_test.rs` | 8 | GET /feed, GET /trending, GET /rankings/* |
| `tests/social_auth_test.rs` | 6 | GET/DELETE /users/me/social-accounts |

### Test Count

| File | Tests |
|------|-------|
| auth_test.rs (existing) | 10 |
| user_test.rs | 14 |
| challenge_test.rs | 10 |
| answer_test.rs | 17 |
| feed_test.rs | 8 |
| social_auth_test.rs | 6 |
| **Integration total** | **65** |
| Unit tests in src/ (existing) | 26 |
| **Grand total** | **91** |

## How to Run

```bash
cd sys/backend/app
docker run --rm --network backend_gengoka-network \
  -v $(pwd):/app -w /app \
  -e TEST_DB_HOST=gengoka-db \
  rust:latest cargo test
```

## Design Decisions

- All integration tests use `#[serial]` + `clean_db()` for test isolation
- Social auth endpoints calling external providers (social_login, link_account) are skipped at integration level — already covered by 26 wiremock-based unit tests
- `get_linked_accounts` and `unlink_account` are tested by inserting data directly into DB
- Test helpers use direct SQL inserts for speed where API calls aren't the subject under test
