# Phase 1 Step 2: iOS ソーシャル認証実装

**実装日:** 2026-02-17
**ステータス:** 完了（Apple完全動作可能 / Google・LINE SDK統合は残作業）

---

## 概要

iOS アプリの LoginView / RegisterView にソーシャルログインボタン（Google, Apple, LINE）を追加。
Apple Sign-In は `AuthenticationServices` 標準フレームワークで完全実装済み。
Google / LINE はボタンUI + Backend連携コード実装済み、SDK統合のみ残作業。

---

## 新規作成ファイル

| ファイル | 内容 |
|---------|------|
| `Services/SocialAuthService.swift` | `SocialAuthProvider` enum（google/apple/line）、`SocialAuthResult` 構造体、`AppleSignInDelegate`（ASAuthorizationControllerDelegate実装）、`SocialAuthError` エラー型 |
| `Views/Home/SocialLoginButton.swift` | ソーシャルログイン共通ボタンコンポーネント（アイコン、タイトル、背景色、ボーダー、ローディング状態対応） |

## 変更ファイル

| ファイル | 変更内容 |
|---------|---------|
| `Services/AuthService.swift` | `socialLogin(result:)` メソッド追加。`SocialAuthResult` を受け取り `SocialLoginRequest` を構築して `/auth/social` に送信、レスポンスを `saveAuth()` で保存 |
| `Services/APIEndpoints.swift` | `.socialLogin` case追加。path: `/auth/social`、method: POST |
| `Views/Home/AuthModels.swift` | `SocialLoginRequest` モデル追加（provider, id_token, access_token, device_info） |
| `Views/Home/LoginView.swift` | ソーシャルログインボタン3つ追加、`handleSocialLogin(provider:)` メソッド追加、`isSocialLoading` 状態追加 |
| `Views/Home/RegisterView.swift` | 同上（ボタンラベルが「〜で登録」に変更） |

---

## UI構成

LoginView / RegisterView 共通:
```
[メールフォーム]
[ログイン/登録ボタン]

────── または ──────

[G] Googleでログイン      ← 白背景 + グレーボーダー
[🍎] Appleでログイン       ← 黒背景 + 白文字
[LINE] LINEでログイン      ← 緑(#06C755)背景 + 白文字
```

---

## Apple Sign-In 実装詳細

```
LoginView / RegisterView
    │
    └─ handleSocialLogin(provider: .apple)
         │
         └─ AppleSignInDelegate.signIn()
              │
              ├─ ASAuthorizationAppleIDProvider.createRequest()
              │   └─ requestedScopes: [.fullName, .email]
              │
              ├─ ASAuthorizationController.performRequests()
              │
              ├─ 成功 → identityToken(JWT) を取得
              │   └─ SocialAuthResult(provider: .apple, idToken: jwt)
              │
              └─ キャンセル → SocialAuthError.cancelled（エラー非表示）
                   │
    └─ AuthService.socialLogin(result:)
         │
         └─ POST /api/v1/auth/social
              body: { provider: "apple", id_token: "jwt..." }
              │
              └─ AuthResponse → saveAuth() → onLoginSuccess()
```

---

## SocialLoginRequest モデル

```swift
struct SocialLoginRequest: Codable {
    let provider: String       // "google" | "apple" | "line"
    let idToken: String?       // Google / Apple
    let accessToken: String?   // LINE
    let deviceInfo: String?    // "iOS"
}
```

JSON エンコード時のキーマッピング:
- `idToken` → `id_token`
- `accessToken` → `access_token`
- `deviceInfo` → `device_info`

---

## エラーハンドリング

| エラー | 動作 |
|--------|------|
| `SocialAuthError.cancelled` | エラー非表示（ユーザーが自発的にキャンセル） |
| `SocialAuthError.invalidCredential` | 「認証情報の取得に失敗しました」表示 |
| `SocialAuthError.notConfigured(provider)` | 「{provider}の設定が完了していません」表示 |
| `SocialAuthError.providerError(message)` | 「認証エラー: {message}」表示 |
| `NetworkError` | 既存のネットワークエラーメッセージ表示 |

---

## 残作業

### Google Sign-In SDK 統合
1. Xcode → File → Add Package Dependencies → `https://github.com/google/GoogleSignIn-iOS`
2. `Info.plist` に `GIDClientID` と URL Scheme 追加
3. `SocialAuthService.swift` に Google Sign-In フロー実装
4. `LoginView` / `RegisterView` の `.google` case を実装に置き換え

### LINE SDK 統合
1. Xcode → File → Add Package Dependencies → `https://github.com/nicklama/line-sdk-ios-swift-spm` (SPM対応fork)
2. `Info.plist` に `LineSDKConfig` (Channel ID) と URL Scheme 追加
3. `SocialAuthService.swift` に LINE Login フロー実装
4. `LoginView` / `RegisterView` の `.line` case を実装に置き換え

### Xcode プロジェクト設定
- Signing & Capabilities → 「Sign in with Apple」capability 追加
