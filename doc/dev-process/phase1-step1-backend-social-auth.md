# Phase 1 Step 1: Backend ソーシャル認証実装

**実装日:** 2026-02-17
**ステータス:** 完了

---

## 概要

Google / Apple / LINE のソーシャル認証を Backend に実装。
モバイルアプリから `POST /api/v1/auth/social` にプロバイダートークンを送信し、
既存の JWT 認証フローと同一形式でレスポンスを返す。

---

## 新規作成ファイル

| ファイル | 内容 |
|---------|------|
| `src/handlers/social_auth.rs` | `social_login()` ハンドラー、`find_or_create_user()` ユーザー検索/作成ロジック |
| `src/services/social_auth/mod.rs` | モジュール定義、re-export |
| `src/services/social_auth/google.rs` | Google JWKS → RS256署名検証 → audience/issuer検証 → claims抽出 |
| `src/services/social_auth/apple.rs` | Apple JWKS → RS256署名検証 → audience/issuer検証 → claims抽出 |
| `src/services/social_auth/line.rs` | verify API でトークン有効性確認 → profile API でユーザー情報取得 |

## 変更ファイル

| ファイル | 変更内容 |
|---------|---------|
| `src/config/mod.rs` | `SocialAuthConfig` 構造体追加、`Config` に `social_auth` フィールド追加、環境変数読み込み追加 |
| `src/models/mod.rs` | `SocialLoginRequest`, `SocialUserInfo` モデル追加 |
| `src/db/mod.rs` | `user_social_accounts` テーブル + インデックスのマイグレーション追加 |
| `src/handlers/mod.rs` | `social_auth` モジュール登録 |
| `src/services/mod.rs` | `social_auth` モジュール登録 |
| `src/routes/mod.rs` | `/auth/social` ルート追加 |

---

## APIエンドポイント

```
POST /api/v1/auth/social
```

### リクエスト

```json
{
    "provider": "google" | "apple" | "line",
    "id_token": "string",        // Google / Apple
    "access_token": "string",    // LINE
    "device_info": "string"      // 任意
}
```

### レスポンス（既存 AuthTokens と同一）

```json
{
    "success": true,
    "data": {
        "access_token": "jwt...",
        "refresh_token": "token...",
        "expires_in": 1800,
        "user": {
            "id": "uuid",
            "name": "string",
            "avatar": "string|null"
        }
    }
}
```

---

## データベース

```sql
CREATE TABLE user_social_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(20) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    provider_email VARCHAR(255),
    provider_name VARCHAR(255),
    provider_avatar VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_social_accounts_user ON user_social_accounts(user_id);
CREATE INDEX idx_social_accounts_provider ON user_social_accounts(provider, provider_user_id);
```

---

## ユーザー検索/作成ロジック

```
1. user_social_accounts で provider + provider_user_id を検索
   └─ 見つかった → プロバイダー情報を更新 → ログイン

2. 見つからない → users テーブルで同一メールアドレスを検索
   └─ 見つかった → 既存ユーザーにソーシャルアカウントをリンク → ログイン

3. どちらも見つからない → 新規ユーザー作成 → ソーシャルアカウントをリンク → ログイン

4. JWT access_token + refresh_token を発行して返却
```

---

## プロバイダー別 検証方式

| プロバイダー | 検証方式 | 検証元URL |
|-------------|---------|-----------|
| Google | IDトークン → JWKS(RS256) → audience + issuer 検証 | `googleapis.com/oauth2/v3/certs` |
| Apple | IDトークン → JWKS(RS256) → audience + issuer 検証 | `appleid.apple.com/auth/keys` |
| LINE | access_token → verify API → profile API | `api.line.me/oauth2/v2.1/verify`, `api.line.me/v2/profile` |

---

## 環境変数（.env に追加が必要）

```env
# Google OAuth
GOOGLE_CLIENT_ID_IOS=xxx.apps.googleusercontent.com
GOOGLE_CLIENT_ID_ANDROID=xxx.apps.googleusercontent.com

# Apple Sign-In
APPLE_CLIENT_ID=app.dev.gengoka
APPLE_TEAM_ID=XXXXXXXXXX

# LINE Login
LINE_CHANNEL_ID=1234567890
LINE_CHANNEL_SECRET=xxxxxxxxxxxxxxxx
```

---

## 備考

- `password_hash` カラムは既に NULLABLE のため ALTER 不要
- 既存の `jsonwebtoken` クレートを Google/Apple の JWK 検証にも使用（追加依存なし）
- 既存の `reqwest` クレートを LINE API 呼出に使用（追加依存なし）
- Rust がローカルに未インストールのため `cargo check` 未実施。デプロイ環境でのビルド確認が必要
