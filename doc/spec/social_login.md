# Social Login Specification

Multi-platform social authentication supporting Google, Apple, and LINE across iOS, Android, and Web.

---

## Backend API

### Endpoint

`POST /api/v1/auth/social`

### Request

```json
{
  "provider": "google" | "apple" | "line",
  "id_token": "...",        // Google, Apple
  "access_token": "...",    // LINE
  "device_info": "iOS" | "Android" | "Web"
}
```

### Response

```json
{
  "access_token": "jwt...",
  "refresh_token": "uuid...",
  "expires_in": 1800,
  "user": {
    "id": "uuid",
    "name": "string",
    "avatar": "url or null"
  }
}
```

### User Resolution Flow

1. Find existing `user_social_accounts` by `(provider, provider_user_id)` → use linked user
2. If no link found, find `users` by email → link social account to existing user
3. If no user found → create new user + link social account
   - Display name: provided name → email prefix → `user_{provider_user_id[:8]}`

### Account Linking Endpoints

| Endpoint | Description |
|---|---|
| `GET /api/v1/users/me/social-accounts` | List linked social accounts |
| `POST /api/v1/users/me/social-accounts` | Link a new social account |
| `DELETE /api/v1/users/me/social-accounts/{provider}` | Unlink (requires another auth method) |

---

## Provider Verification

### Google — JWT (RS256)

1. Fetch JWKS from `https://www.googleapis.com/oauth2/v3/certs`
2. Match `kid` in JWT header → decode with RSA public key
3. Validate audience against **3 client IDs** (web, iOS, Android)
4. Validate issuer: `accounts.google.com` or `https://accounts.google.com`
5. Extract: `sub`, `email`, `name`, `picture`

### Apple — JWT (RS256)

1. Fetch JWKS from `https://appleid.apple.com/auth/keys`
2. Match `kid` → decode with RSA public key
3. Validate audience against `apple_client_id` (iOS) and `apple_client_id_web` (Web)
4. Validate issuer: `https://appleid.apple.com`
5. Extract: `sub`, `email` (name only available on first sign-in)
6. Avatar: not provided by Apple

### LINE — Access Token

1. Verify token at `https://api.line.me/oauth2/v2.1/verify`
2. Validate `client_id` matches and `expires_in > 0`
3. Fetch profile from `https://api.line.me/v2/profile` with `Authorization: Bearer`
4. Extract: `userId`, `displayName`, `pictureUrl`
5. Email: not provided by LINE profile API

---

## Backend Configuration

| Env Var | Description | Default |
|---|---|---|
| `GOOGLE_CLIENT_ID_WEB` | Google Web client ID | — |
| `GOOGLE_CLIENT_ID_IOS` | Google iOS client ID | — |
| `GOOGLE_CLIENT_ID_ANDROID` | Google Android client ID | — |
| `APPLE_CLIENT_ID` | Apple iOS Bundle ID | `app.gengoka` |
| `APPLE_CLIENT_ID_WEB` | Apple Web Service ID | — |
| `APPLE_TEAM_ID` | Apple Team ID | — |
| `LINE_CHANNEL_ID` | LINE Channel ID | — |
| `LINE_CHANNEL_SECRET` | LINE Channel Secret | — |

---

## Platform: iOS

### Dependencies

- **Google**: Google Sign-In SDK (`GoogleSignIn`)
- **Apple**: AuthenticationServices (native)
- **LINE**: LINE SDK (`LineSDK`)

### Flows

#### Google
```
GoogleSignIn SDK → GIDSignIn.signIn(withPresenting:)
→ id_token (JWT) from result.user.idToken.tokenString
→ POST /auth/social { provider: "google", id_token }
```

#### Apple
```
ASAuthorizationAppleIDProvider → request scopes (.fullName, .email)
→ ASAuthorizationController presents native UI
→ credential.identityToken (JWT)
→ POST /auth/social { provider: "apple", id_token }
```

#### LINE
```
LineSDK → LoginManager.shared.login(permissions: [.profile])
→ accessToken from loginResult.lineCredential.accessToken.tokenString
→ POST /auth/social { provider: "line", access_token }
```

---

## Platform: Android

### Dependencies

- **Google**: Android Credential Manager + Google Identity Services
- **LINE**: LINE SDK
- **Apple**: Not implemented (iOS/Web only)

### Flows

#### Google
```
CredentialManager → GetGoogleIdOption (saved credentials)
→ fallback: GetSignInWithGoogleOption (bottom sheet)
→ GoogleIdTokenCredential.idToken (JWT)
→ POST /auth/social { provider: "google", id_token }
```

#### LINE
```
LineLoginApi.getLoginIntent(context, CHANNEL_ID, params)
→ startActivityForResult
→ LineLoginApi.getLoginResultFromIntent(data)
→ accessToken from result.lineCredential.accessToken.tokenString
→ POST /auth/social { provider: "line", access_token }
```

---

## Platform: Web (Next.js)

### Dependencies

- **Google**: `@react-oauth/google` (popup, built-in button)
- **Apple**: OAuth redirect (no SDK)
- **LINE**: OAuth redirect (no SDK)

### Environment Variables

```env
NEXT_PUBLIC_GOOGLE_CLIENT_ID=...
NEXT_PUBLIC_APPLE_CLIENT_ID=app.gengoka.web
NEXT_PUBLIC_LINE_CHANNEL_ID=...
LINE_CHANNEL_SECRET=...  # server-side only
```

### Flows

#### Google — Popup

```
<GoogleLogin> component → onSuccess callback
→ credentialResponse.credential (JWT id_token)
→ POST /auth/social { provider: "google", id_token }
```

No redirect, no API route needed. Token obtained directly in browser.

#### Apple — OAuth Redirect (form_post)

```
1. Button click → generate state → sessionStorage('apple_oauth_state')
2. Redirect to:
   https://appleid.apple.com/auth/authorize?
     response_type=code%20id_token
     &response_mode=form_post
     &client_id={APPLE_CLIENT_ID}
     &redirect_uri={origin}/api/auth/apple/callback
     &state={state}
     &scope=name%20email

3. Apple POST → /api/auth/apple/callback (route.ts)
   - Extract id_token, state from form body
   - Redirect to /apple-callback?id_token=...&state=...

4. /apple-callback (page.tsx)
   - Validate state against sessionStorage
   - POST /auth/social { provider: "apple", id_token }
   - Navigate to /home
```

Note: Apple requires HTTPS. `localhost` does not work. Use ngrok or deployed environment.

#### LINE — OAuth Redirect (authorization code)

```
1. Button click → generate state → sessionStorage('line_oauth_state')
2. Redirect to:
   https://access.line.me/oauth2/v2.1/authorize?
     response_type=code
     &client_id={CHANNEL_ID}
     &redirect_uri={origin}/api/auth/line/callback
     &state={state}
     &scope=profile

3. LINE GET → /api/auth/line/callback (route.ts)
   - Exchange code for access_token at LINE token API
     (using server-side LINE_CHANNEL_SECRET)
   - Redirect to /line-callback?access_token=...&state=...

4. /line-callback (page.tsx)
   - Validate state against sessionStorage
   - POST /auth/social { provider: "line", access_token }
   - Navigate to /home
```

### Web Routes

| Route | Type | Purpose |
|---|---|---|
| `/api/auth/apple/callback` | API (POST) | Receive Apple form_post, extract id_token |
| `/api/auth/line/callback` | API (GET) | Exchange LINE code for access_token |
| `/apple-callback` | Page | Validate state, call socialLogin with id_token |
| `/line-callback` | Page | Validate state, call socialLogin with access_token |

### Middleware

Public paths (no auth required): `/login`, `/register`, `/api/auth`, `/line-callback`, `/apple-callback`

---

## Security

- **JWT Verification**: Google/Apple tokens verified with provider JWKS (RS256 public keys)
- **CSRF Protection**: OAuth redirect flows use cryptographic `state` parameter (32 random bytes → 64-char hex)
- **Token Rotation**: Refresh tokens rotated on each refresh call; old token deleted
- **Audience Validation**: Google accepts 3 client IDs (web/iOS/Android); Apple accepts 2 (iOS/Web)
- **Issuer Validation**: JWT issuer checked against known provider URLs
- **Channel ID Validation**: LINE access tokens validated against configured channel ID
- **Server-Side Secrets**: LINE channel secret never exposed to client
