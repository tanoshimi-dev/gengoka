# Gengoka - Development Plan

## 開発プロセス概要

本ドキュメントは Gengoka アプリの今後の開発計画を定義する。
各フェーズは依存関係を考慮した順序で構成されている。

---

## Phase 1: ソーシャル認証の追加（Google / Apple / LINE）

### 1.1 概要

現在のメール/パスワード認証に加え、以下のソーシャル認証プロバイダーを追加する。

| プロバイダー | iOS | Android | 備考 |
|-------------|-----|---------|------|
| Google | o | o | 両プラットフォーム対応 |
| Apple | o | - | iOSのみ（Apple要件） |
| LINE | o | o | 日本市場向け、両プラットフォーム対応 |

### 1.2 認証フロー

```
[モバイルアプリ]
    │
    ├─ Google Sign-In SDK / Apple AuthenticationServices / LINE SDK
    │   └─ ユーザー認証 → IDトークン/認可コード取得
    │
    ▼
[Backend API]  POST /api/v1/auth/social
    │
    ├─ IDトークン/認可コードの検証
    │   ├─ Google: google-auth ライブラリでIDトークン検証
    │   ├─ Apple: Apple公開鍵でJWT検証
    │   └─ LINE: LINE APIでアクセストークン検証 → プロフィール取得
    │
    ├─ ユーザー検索/作成
    │   ├─ provider + provider_user_id で既存ユーザー検索
    │   ├─ メールアドレスで既存ユーザー検索（アカウントリンク）
    │   └─ 新規の場合: ユーザー自動作成
    │
    └─ JWT発行（既存のトークン発行フローと同一）
        └─ access_token + refresh_token を返却
```

### 1.3 Backend 実装

#### 1.3.1 データベース変更

```sql
-- ソーシャル認証リンクテーブル
CREATE TABLE user_social_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(20) NOT NULL,          -- 'google', 'apple', 'line'
    provider_user_id VARCHAR(255) NOT NULL,  -- プロバイダー側のユーザーID
    provider_email VARCHAR(255),             -- プロバイダーから取得したメール
    provider_name VARCHAR(255),              -- プロバイダーから取得した名前
    provider_avatar VARCHAR(500),            -- プロバイダーから取得したアバターURL
    access_token_hash VARCHAR(255),          -- プロバイダーのアクセストークン（暗号化）
    refresh_token_hash VARCHAR(255),         -- プロバイダーのリフレッシュトークン（暗号化）
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_social_accounts_user ON user_social_accounts(user_id);
CREATE INDEX idx_social_accounts_provider ON user_social_accounts(provider, provider_user_id);

-- usersテーブルにpassword_hashをNULLABLE化（ソーシャル認証のみユーザー対応）
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;
```

#### 1.3.2 新規APIエンドポイント

```
POST /api/v1/auth/social
```

**リクエスト:**
```json
{
    "provider": "google" | "apple" | "line",
    "id_token": "string",           // Google / Apple
    "access_token": "string",       // LINE
    "authorization_code": "string", // Apple (初回)
    "nonce": "string"               // Apple
}
```

**レスポンス:** 既存の `AuthResponse` と同一
```json
{
    "data": {
        "access_token": "jwt...",
        "refresh_token": "token...",
        "expires_in": 1800,
        "user": { "id": "uuid", "email": "...", "name": "..." }
    }
}
```

#### 1.3.3 Backend実装ファイル

| ファイル | 変更内容 |
|---------|---------|
| `Cargo.toml` | `jsonwebtoken`(Apple JWT検証), `reqwest`(LINE API呼出) 追加 |
| `src/db/mod.rs` | `user_social_accounts` テーブルのマイグレーション追加 |
| `src/handlers/auth.rs` | `social_login()` ハンドラー追加 |
| `src/services/social_auth/mod.rs` | ソーシャル認証サービス（新規） |
| `src/services/social_auth/google.rs` | Google IDトークン検証（新規） |
| `src/services/social_auth/apple.rs` | Apple IDトークン検証（新規） |
| `src/services/social_auth/line.rs` | LINE アクセストークン検証（新規） |
| `src/models/social_auth.rs` | ソーシャル認証モデル（新規） |
| `src/routes/mod.rs` | `/auth/social` ルート追加 |
| `src/config/mod.rs` | OAuth設定（Client ID等）追加 |

#### 1.3.4 プロバイダー別 検証ロジック

**Google:**
- IDトークンをGoogleの公開鍵で検証（JWK）
- `https://www.googleapis.com/oauth2/v3/certs` から公開鍵取得
- audience（Client ID）の一致確認
- email, name, picture を抽出

**Apple:**
- IDトークンをAppleの公開鍵で検証（JWK）
- `https://appleid.apple.com/auth/keys` から公開鍵取得
- audience（Client ID）、issuer の一致確認
- email, name（初回のみ）を抽出
- nonce検証

**LINE:**
- アクセストークンで LINE Profile API を呼出
- `https://api.line.me/v2/profile` → userId, displayName, pictureUrl
- `https://api.line.me/oauth2/v2.1/verify` → トークン有効性確認
- Channel IDの一致確認

### 1.4 iOS 実装

#### 1.4.1 依存関係

| ライブラリ | 用途 | 導入方法 |
|-----------|------|---------|
| GoogleSignIn | Google認証 | SPM |
| AuthenticationServices | Apple認証 | 標準フレームワーク |
| LineSDKSwift | LINE認証 | SPM |

#### 1.4.2 実装ファイル

| ファイル | 変更内容 |
|---------|---------|
| `Services/AuthService.swift` | `socialLogin()` メソッド追加 |
| `Services/SocialAuthService.swift` | Google/Apple/LINE SDK連携（新規） |
| `Services/APIEndpoints.swift` | `.socialLogin` エンドポイント追加 |
| `Views/Home/AuthModels.swift` | `SocialLoginRequest` モデル追加 |
| `Views/Home/LoginView.swift` | ソーシャルログインボタン追加 |
| `Views/Home/RegisterView.swift` | ソーシャルログインボタン追加 |
| `Info.plist` | URL Schemes、LSApplicationQueriesSchemes 設定 |
| `Gengoka.entitlements` | Sign in with Apple capability |

#### 1.4.3 UI変更

LoginView / RegisterView に以下を追加:
```
─────── または ───────

[G] Googleでログイン        ← 両画面共通
[🍎] Appleでログイン        ← 両画面共通
[LINE] LINEでログイン       ← 両画面共通
```

### 1.5 Android 実装

#### 1.5.1 依存関係

| ライブラリ | 用途 |
|-----------|------|
| `com.google.android.gms:play-services-auth` | Google Sign-In |
| `androidx.credentials:credentials` | Credential Manager（Google推奨） |
| `com.linecorp.linesdk:linesdk` | LINE SDK |

#### 1.5.2 実装ファイル

| ファイル | 変更内容 |
|---------|---------|
| `build.gradle.kts` | 依存関係追加 |
| `data/api/GengokApi.kt` | `socialLogin()` エンドポイント追加 |
| `data/dto/SocialAuthDto.kt` | リクエスト/レスポンスDTO（新規） |
| `data/repository/AuthRepositoryImpl.kt` | `socialLogin()` 実装追加 |
| `domain/repository/AuthRepository.kt` | `socialLogin()` インターフェース追加 |
| `presentation/screens/auth/AuthViewModel.kt` | ソーシャルログイン処理追加 |
| `presentation/screens/auth/LoginScreen.kt` | ソーシャルログインボタン追加 |
| `presentation/screens/auth/RegisterScreen.kt` | ソーシャルログインボタン追加 |
| `presentation/screens/auth/SocialAuthHelper.kt` | SDK連携ヘルパー（新規） |
| `AndroidManifest.xml` | intent-filter、permissions追加 |

### 1.6 開発ステップ

```
Step 1: DB マイグレーション + Backend ソーシャル認証エンドポイント
         ├─ user_social_accounts テーブル作成
         ├─ POST /api/v1/auth/social ハンドラー
         └─ Google/Apple/LINE 検証サービス実装
              │
Step 2: iOS ソーシャル認証実装
         ├─ Google Sign-In SDK 統合
         ├─ Sign in with Apple 統合
         ├─ LINE SDK 統合
         └─ Login/Register UI更新
              │
Step 3: Android ソーシャル認証実装
         ├─ Google Credential Manager 統合
         ├─ LINE SDK 統合
         └─ Login/Register UI更新
              │
Step 4: アカウントリンク機能
         ├─ 既存メールアカウントとソーシャルアカウントの紐付け
         ├─ マイページからのアカウント連携管理
         └─ 複数プロバイダーリンク対応
              │
Step 5: テスト・検証
         ├─ 各プロバイダーでの新規登録フロー
         ├─ 既存ユーザーとのアカウントリンク
         ├─ エッジケース（メール重複、トークン期限切れ等）
         └─ UI/UXレビュー
```

### 1.7 外部設定（各プロバイダーコンソール）

| プロバイダー | コンソール | 必要な設定 |
|-------------|-----------|-----------|
| Google | Google Cloud Console | OAuth 2.0 Client ID（iOS/Android/Web）、リダイレクトURI |
| Apple | Apple Developer Portal | App ID に Sign in with Apple 有効化、Service ID |
| LINE | LINE Developers Console | チャネル作成、コールバックURL、LINE Login設定 |

### 1.8 環境変数（Backend追加分）

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

## Phase 2: テスト基盤の構築

### 2.1 概要

現在テストファイルが一切存在しないため、ユニットテストとE2Eテストの基盤を構築する。

### 2.2 Backend ユニットテスト

#### 2.2.1 テスト構成

```
sys/backend/app/
├── src/
│   └── ...
└── tests/                          # 統合テスト
    ├── common/
    │   └── mod.rs                  # テストヘルパー（DBセットアップ、テストユーザー作成）
    ├── auth_test.rs                # 認証テスト
    ├── social_auth_test.rs         # ソーシャル認証テスト
    ├── challenge_test.rs           # チャレンジテスト
    ├── answer_test.rs              # 回答テスト
    ├── user_test.rs                # ユーザーテスト
    ├── feed_test.rs                # フィードテスト
    └── ranking_test.rs             # ランキングテスト
```

各 `src/` モジュール内にもユニットテストを配置:
```rust
// src/handlers/auth.rs 末尾
#[cfg(test)]
mod tests {
    use super::*;
    // ハンドラーのユニットテスト
}
```

#### 2.2.2 テストカバレッジ目標

| モジュール | テスト対象 | 優先度 |
|-----------|-----------|--------|
| auth handler | 登録、ログイン、トークンリフレッシュ、ログアウト | 高 |
| social auth service | Google/Apple/LINE トークン検証（モック） | 高 |
| answer handler | 回答CRUD、いいね、コメント | 中 |
| challenge handler | チャレンジ取得、デイリーチャレンジ | 中 |
| user handler | プロフィール、フォロー、統計 | 中 |
| ranking handler | 各種ランキング計算 | 低 |
| middleware | JWT検証、認可チェック | 高 |

#### 2.2.3 テスト環境

- **テスト用DB**: `gengoka_test` データベース（`DATABASE_URL_TEST` 環境変数）
- **テストフレームワーク**: Rust標準 `#[tokio::test]` + `actix-web::test`
- **モック**: `mockall` クレートで外部API（Gemini, Google, Apple, LINE）をモック
- **テストデータ**: `fixtures/` ディレクトリにシードデータ

#### 2.2.4 Cargo.toml 追加

```toml
[dev-dependencies]
actix-rt = "2"
mockall = "0.12"
fake = "2.9"          # テストデータ生成
serial_test = "3"     # テスト直列実行
```

### 2.3 iOS ユニットテスト

#### 2.3.1 テスト構成

```
Gengoka/
├── GengokTests/                      # ユニットテスト
│   ├── Services/
│   │   ├── AuthServiceTests.swift
│   │   ├── SocialAuthServiceTests.swift
│   │   └── APIClientTests.swift
│   ├── ViewModels/
│   │   ├── HomeViewModelTests.swift
│   │   ├── ChallengeViewModelTests.swift
│   │   └── FeedViewModelTests.swift
│   └── Models/
│       ├── AuthModelsTests.swift
│       └── ChallengeModelsTests.swift
│
└── GengokUITests/                    # UIテスト
    ├── AuthenticationUITests.swift
    ├── ChallengeFlowUITests.swift
    └── FeedUITests.swift
```

#### 2.3.2 テストカバレッジ目標

| 対象 | テスト内容 | 優先度 |
|------|-----------|--------|
| AuthService | ログイン、登録、ログアウト、トークンリフレッシュ | 高 |
| SocialAuthService | Google/Apple/LINE SDK呼出（モック） | 高 |
| AuthModels | バリデーションロジック | 高 |
| APIClient | リクエスト構築、エラーハンドリング | 中 |
| HomeViewModel | デイリーチャレンジ取得、状態管理 | 中 |
| ChallengeViewModel | 回答投稿、AIフィードバック表示 | 中 |

#### 2.3.3 テスト手法

- **ユニットテスト**: XCTest フレームワーク
- **モック**: Protocol-based Dependency Injection（APIClientをProtocol化）
- **UIテスト**: XCUITest フレームワーク
- **非同期テスト**: `async/await` + XCTest expectations

### 2.4 Android ユニットテスト

#### 2.4.1 テスト構成

```
app/src/
├── test/                              # ローカルユニットテスト
│   └── kotlin/app/dev/gengoka/
│       ├── data/repository/
│       │   ├── AuthRepositoryImplTest.kt
│       │   ├── ChallengeRepositoryImplTest.kt
│       │   └── UserRepositoryImplTest.kt
│       ├── presentation/screens/auth/
│       │   └── AuthViewModelTest.kt
│       └── core/network/
│           └── TokenManagerTest.kt
│
└── androidTest/                       # Instrumented テスト
    └── kotlin/app/dev/gengoka/
        ├── presentation/screens/auth/
        │   ├── LoginScreenTest.kt
        │   └── RegisterScreenTest.kt
        └── EndToEndTest.kt
```

#### 2.4.2 テストカバレッジ目標

| 対象 | テスト内容 | 優先度 |
|------|-----------|--------|
| AuthRepositoryImpl | login, register, refresh, socialLogin | 高 |
| AuthViewModel | 状態管理、エラーハンドリング | 高 |
| TokenManager | トークン保存/取得/削除 | 高 |
| ChallengeRepository | チャレンジ取得、回答投稿 | 中 |
| UserRepository | プロフィール、フォロー | 中 |

#### 2.4.3 テスト依存関係

```kotlin
// build.gradle.kts 追加
testImplementation("junit:junit:4.13.2")
testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.7.3")
testImplementation("io.mockk:mockk:1.13.9")
testImplementation("app.cash.turbine:turbine:1.0.0")  // StateFlowテスト

androidTestImplementation("androidx.compose.ui:ui-test-junit4")
androidTestImplementation("com.google.dagger:hilt-android-testing:2.50")
```

### 2.5 E2Eテスト

#### 2.5.1 テスト対象フロー

| # | テストフロー | 対象 | 優先度 |
|---|-------------|------|--------|
| 1 | メール登録 → ログイン → ログアウト | Backend + Mobile | 高 |
| 2 | Google ソーシャルログイン → プロフィール確認 | Backend + Mobile | 高 |
| 3 | Apple ソーシャルログイン → プロフィール確認 | Backend + iOS | 高 |
| 4 | LINE ソーシャルログイン → プロフィール確認 | Backend + Mobile | 高 |
| 5 | デイリーチャレンジ表示 → 回答投稿 → AIフィードバック確認 | Backend + Mobile | 高 |
| 6 | フィード表示 → いいね → コメント投稿 | Backend + Mobile | 中 |
| 7 | ユーザーフォロー → フォロー中フィード確認 | Backend + Mobile | 中 |
| 8 | ランキング表示 → 正確性確認 | Backend + Mobile | 低 |

#### 2.5.2 Backend E2Eテスト

- **フレームワーク**: `actix-web::test` + テスト用HTTPクライアント
- **アプローチ**: テストサーバー起動 → APIリクエスト → レスポンス検証
- **データベース**: テスト毎にトランザクションロールバック

```rust
// tests/e2e/auth_flow_test.rs
#[actix_web::test]
async fn test_full_auth_flow() {
    // 1. Register
    // 2. Login with same credentials
    // 3. Access protected endpoint with token
    // 4. Refresh token
    // 5. Logout
    // 6. Verify token is invalid
}
```

#### 2.5.3 Mobile E2Eテスト

**iOS**: XCUITest
```swift
// GengokUITests/AuthenticationUITests.swift
func testLoginFlow() {
    // 1. アプリ起動 → ログイン画面表示確認
    // 2. メール/パスワード入力
    // 3. ログインボタンタップ
    // 4. ホーム画面表示確認
}
```

**Android**: Compose UI Testing + Hilt
```kotlin
// androidTest/.../EndToEndTest.kt
@Test
fun testLoginFlow() {
    // 1. ログイン画面表示確認
    // 2. メール/パスワード入力
    // 3. ログインボタンタップ
    // 4. ホーム画面表示確認
}
```

### 2.6 テスト開発ステップ

```
Step 1: Backend テスト基盤構築
         ├─ テスト用DB設定、テストヘルパー作成
         ├─ 認証ハンドラーのユニットテスト
         └─ 認証フローのE2Eテスト
              │
Step 2: Backend テスト拡充
         ├─ ソーシャル認証テスト（モック使用）
         ├─ チャレンジ/回答ハンドラーテスト
         └─ ユーザー/フィード/ランキングテスト
              │
Step 3: iOS テスト基盤構築
         ├─ テストターゲット追加
         ├─ APIClient Protocol化（DI対応）
         ├─ AuthService ユニットテスト
         └─ 認証UIテスト
              │
Step 4: Android テスト基盤構築
         ├─ テスト依存関係追加
         ├─ AuthRepository ユニットテスト
         ├─ AuthViewModel ユニットテスト
         └─ 認証画面 Compose UIテスト
              │
Step 5: テスト拡充 + CI連携
         ├─ 各プラットフォームのテストカバレッジ拡大
         └─ CI/CDパイプラインでの自動テスト実行
```

---

## Phase 3以降: 今後の機能開発（優先順位順）

| Phase | 機能 | 概要 |
|-------|------|------|
| 3 | パスワードリセット | メール送信によるパスワードリセットフロー |
| 4 | プッシュ通知 | FCM(Android) / APNs(iOS) によるデイリーチャレンジ通知 |
| 5 | ゲーミフィケーション | ストリーク報酬、バッジ、レベルシステム |
| 6 | シェアカード | 回答を画像化してSNSシェア |
| 7 | 難易度レベル | 初級/中級/上級の文字数制限バリエーション |
| 8 | Web フロントエンド | Next.js によるWebアプリ |
| 9 | 多言語対応 | 英語、中国語、韓国語対応 |
| 10 | 課金/プレミアム | サブスクリプションモデル導入 |

---

## 開発スケジュール目安

```
Phase 1: ソーシャル認証
├── Step 1 (Backend)        ─── 実装
├── Step 2 (iOS)            ─── 実装
├── Step 3 (Android)        ─── 実装
├── Step 4 (アカウントリンク) ─── 実装
└── Step 5 (テスト・検証)    ─── QA

Phase 2: テスト基盤
├── Step 1 (Backend基盤)     ─── 実装
├── Step 2 (Backend拡充)     ─── 実装
├── Step 3 (iOS)             ─── 実装
├── Step 4 (Android)         ─── 実装
└── Step 5 (CI連携)          ─── 実装
```

---

## 備考

- 各PhaseのStep完了時にコードレビューを実施
- ソーシャル認証の各プロバイダーは独立して開発・リリース可能
- テストはソーシャル認証実装と並行して進められる（Phase 1 Step 1完了後）
- E2Eテストはソーシャル認証のテストも含む
