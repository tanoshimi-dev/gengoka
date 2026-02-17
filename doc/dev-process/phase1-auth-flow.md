# Phase 1: Social Authentication Flow

## Google Sign-In

### Real Flow (Production)

```mermaid
sequenceDiagram
    participant User
    participant App as Mobile App (iOS/Android)
    participant Backend
    participant Google as Google APIs

    User->>App: Tap "Sign in with Google"
    App->>Google: Google Sign-In SDK
    Google-->>App: ID Token (JWT)
    App->>Backend: POST /api/auth/social {provider: "google", token}
    Backend->>Google: GET googleapis.com/oauth2/v3/certs (JWKS)
    Google-->>Backend: Public keys (JWK set)
    Backend->>Backend: Decode JWT header, find matching kid
    Backend->>Backend: Verify signature (RS256), audience, issuer, expiry
    Backend->>Backend: Extract claims (sub, email, name, picture)
    Backend->>Backend: Find or create user in DB
    Backend-->>App: {access_token, refresh_token, user}
    App-->>User: Signed in
```

### Test Flow (Mock)

```mermaid
sequenceDiagram
    participant Test
    participant Code as verify_google_token()
    participant Mock as MockServer (wiremock)

    Test->>Test: Generate RSA key pair
    Test->>Test: Build JWKS JSON from public key
    Test->>Mock: Mount GET /jwks -> JWKS response
    Test->>Test: Sign JWT with private key (RS256)
    Test->>Code: verify_google_token(jwt, config{jwks_url=mock})
    Code->>Mock: GET /jwks
    Mock-->>Code: JWKS (test public key)
    Code->>Code: Verify signature, audience, issuer, expiry
    Code-->>Test: Ok(SocialUserInfo) or Err
    Test->>Test: Assert result
```

---

## Apple Sign-In

### Real Flow (Production)

```mermaid
sequenceDiagram
    participant User
    participant App as Mobile App (iOS/Android)
    participant Backend
    participant Apple as Apple APIs

    User->>App: Tap "Sign in with Apple"
    App->>Apple: ASAuthorizationController / Web flow
    Apple-->>App: ID Token (JWT)
    App->>Backend: POST /api/auth/social {provider: "apple", token}
    Backend->>Apple: GET appleid.apple.com/auth/keys (JWKS)
    Apple-->>Backend: Public keys (JWK set)
    Backend->>Backend: Decode JWT header, find matching kid
    Backend->>Backend: Verify signature (RS256), audience, issuer, expiry
    Backend->>Backend: Extract claims (sub, email)
    Backend->>Backend: Derive name from email prefix (fallback)
    Backend->>Backend: Find or create user in DB
    Backend-->>App: {access_token, refresh_token, user}
    App-->>User: Signed in
```

### Test Flow (Mock)

```mermaid
sequenceDiagram
    participant Test
    participant Code as verify_apple_token()
    participant Mock as MockServer (wiremock)

    Test->>Test: Generate RSA key pair
    Test->>Test: Build JWKS JSON from public key
    Test->>Mock: Mount GET /jwks -> JWKS response
    Test->>Test: Sign JWT with private key (RS256)
    Test->>Code: verify_apple_token(jwt, config{jwks_url=mock})
    Code->>Mock: GET /jwks
    Mock-->>Code: JWKS (test public key)
    Code->>Code: Verify signature, audience, issuer, expiry
    Code->>Code: Extract email, derive name from email prefix
    Code-->>Test: Ok(SocialUserInfo) or Err
    Test->>Test: Assert result
```

---

## LINE Sign-In

### Real Flow (Production)

```mermaid
sequenceDiagram
    participant User
    participant App as Mobile App (iOS/Android)
    participant Backend
    participant LINE as LINE APIs

    User->>App: Tap "Sign in with LINE"
    App->>LINE: LINE Login SDK
    LINE-->>App: Access Token
    App->>Backend: POST /api/auth/social {provider: "line", token}
    Backend->>LINE: GET api.line.me/oauth2/v2.1/verify?access_token=xxx
    LINE-->>Backend: {client_id, expires_in}
    Backend->>Backend: Validate channel ID match
    Backend->>Backend: Check token not expired
    Backend->>LINE: GET api.line.me/v2/profile (Bearer token)
    LINE-->>Backend: {userId, displayName, pictureUrl}
    Backend->>Backend: Find or create user in DB
    Backend-->>App: {access_token, refresh_token, user}
    App-->>User: Signed in
```

### Test Flow (Mock)

```mermaid
sequenceDiagram
    participant Test
    participant Code as verify_line_token()
    participant Mock as MockServer (wiremock)

    Test->>Mock: Mount GET /verify -> {client_id, expires_in}
    Test->>Mock: Mount GET /profile -> {userId, displayName, pictureUrl}
    Test->>Code: verify_line_token(token, config{urls=mock})
    Code->>Mock: GET /verify?access_token=xxx
    Mock-->>Code: {client_id, expires_in}
    Code->>Code: Validate channel ID, check expiry
    Code->>Mock: GET /profile (Bearer token)
    Mock-->>Code: {userId, displayName, pictureUrl}
    Code-->>Test: Ok(SocialUserInfo) or Err
    Test->>Test: Assert result
```

---

## What Tests Cover vs What They Don't

```mermaid
flowchart LR
    subgraph covered["Covered by Mock Tests"]
        A[JWT signature verification]
        B[Audience / Issuer validation]
        C[Token expiry check]
        D[Key ID matching]
        E[Claims extraction]
        F[Channel ID validation LINE]
        G[Error handling]
    end

    subgraph not_covered["Not Covered - Need Manual/E2E"]
        H[Real provider SDK interaction]
        I[DB user creation/lookup]
        J[Token refresh flow]
        K[Network failures in production]
        L[Provider API format changes]
    end

    style covered fill:#d4edda,stroke:#28a745
    style not_covered fill:#fff3cd,stroke:#ffc107
```
