# Phase 1 Step 3: Android ソーシャル認証実装

**実装日:** 2026-02-17
**ステータス:** 完了（ボタンUI + Backend連携コード実装済み / Google・LINE SDK統合は残作業）

---

## 概要

Android アプリの LoginScreen / RegisterScreen にソーシャルログインボタン（Google, LINE）を追加。
Backend の `POST /api/v1/auth/social` と連携する DTO・Repository・ViewModel を実装済み。
Google Sign-In / LINE SDK の統合のみ残作業。

※ Android は Apple Sign-In 不要のため、Google / LINE の2ボタン構成。

---

## 新規作成ファイル

| ファイル | 内容 |
|---------|------|
| `data/dto/SocialAuthDto.kt` | `SocialLoginRequestDto`（provider, id_token, access_token, device_info）。`@Serializable` + `@SerialName` でスネークケース変換 |
| `presentation/components/SocialLoginButton.kt` | ソーシャルログイン共通ボタンコンポーネント（アイコン、タイトル、背景色、前景色、オプションボーダー、ローディング状態対応） |

## 変更ファイル

| ファイル | 変更内容 |
|---------|----------|
| `data/api/GengokApi.kt` | `@POST("auth/social") socialLogin()` メソッド追加 |
| `domain/repository/AuthRepository.kt` | `socialLogin(provider, idToken, accessToken)` インターフェースメソッド追加 |
| `data/repository/AuthRepositoryImpl.kt` | `socialLogin()` 実装追加。`SocialLoginRequestDto` を構築して API 呼出、`deviceInfo: "Android"` を付与 |
| `presentation/screens/auth/AuthViewModel.kt` | `AuthUiState` に `isSocialLoading` 追加、`socialLogin()` メソッド追加 |
| `presentation/screens/auth/LoginScreen.kt` | `isSocialLoading`, `onSocialLogin` パラメータ追加、Google/LINE ソーシャルログインボタン追加 |
| `presentation/screens/auth/RegisterScreen.kt` | 同上（ボタンラベルが「〜で登録」に変更） |
| `presentation/navigation/NavGraph.kt` | `isSocialLoading`, `onSocialLogin` の ViewModel → Screen 受け渡し追加 |

---

## UI構成

LoginScreen / RegisterScreen 共通:
```
[メールフォーム]
[ログイン/登録ボタン]

────── または ──────

[G] Googleでログイン/登録    ← 白背景 + グレーボーダー
[LINE] LINEでログイン/登録   ← 緑(#06C755)背景 + 白文字
```

---

## アーキテクチャフロー

```
LoginScreen / RegisterScreen
    │
    └─ onSocialLogin("google" | "line")
         │
         └─ AuthViewModel.socialLogin(provider)
              │  isSocialLoading = true
              │
              └─ AuthRepository.socialLogin(provider, idToken, accessToken)
                   │
                   └─ AuthRepositoryImpl.socialLogin()
                        │
                        └─ GengokApi.socialLogin(SocialLoginRequestDto)
                             │
                             └─ POST /api/v1/auth/social
                                  body: { provider, id_token, access_token, device_info: "Android" }
                                  │
                                  └─ ApiResponse<AuthTokensDto> → TokenManager.saveTokens()
```

---

## SocialLoginRequestDto

```kotlin
@Serializable
data class SocialLoginRequestDto(
    val provider: String,           // "google" | "line"
    @SerialName("id_token")
    val idToken: String? = null,    // Google
    @SerialName("access_token")
    val accessToken: String? = null, // LINE
    @SerialName("device_info")
    val deviceInfo: String? = null   // "Android"
)
```

---

## SocialLoginButton コンポーネント

```kotlin
@Composable
fun SocialLoginButton(
    title: String,
    icon: ImageVector,
    backgroundColor: Color,
    contentColor: Color,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    borderColor: Color? = null,
    isLoading: Boolean = false
)
```

- `RoundedCornerShape(12.dp)` で角丸
- ローディング中は `CircularProgressIndicator` 表示
- ボーダーはオプション（Google用に白背景 + グレーボーダー）

---

## iOSとの差異

| 項目 | iOS | Android |
|------|-----|---------|
| Apple Sign-In | あり（ASAuthorizationController） | なし |
| ボタン数 | 3（Google, Apple, LINE） | 2（Google, LINE） |
| deviceInfo | `"iOS"` | `"Android"` |
| アイコン | SF Symbols (`apple.logo`, `g.circle.fill`, `message.fill`) | Material Icons（暫定 `Icons.Filled.Email`） |
| UIフレームワーク | SwiftUI | Jetpack Compose |

---

## 残作業

### Google Sign-In SDK 統合
1. `build.gradle` に `com.google.android.gms:play-services-auth` 追加
2. `google-services.json` に OAuth Client ID 設定
3. `AuthViewModel` or 別Service に Google Sign-In フロー実装（`GoogleSignInClient` → `idToken` 取得）
4. `LoginScreen` / `RegisterScreen` の `onSocialLogin("google")` を実フローに接続
5. ボタンアイコンをGoogleロゴに変更

### LINE SDK 統合
1. `build.gradle` に `com.linecorp.linesdk:linesdk` 追加
2. `AndroidManifest.xml` に LINE Channel ID 設定
3. `AuthViewModel` or 別Service に LINE Login フロー実装（`LineLoginApi` → `accessToken` 取得）
4. `LoginScreen` / `RegisterScreen` の `onSocialLogin("line")` を実フローに接続
5. ボタンアイコンをLINEロゴに変更

### アイコン改善
- Google / LINE のブランドアイコン（SVG/PNG）をリソースに追加し、`Icons.Filled.Email` の暫定アイコンを置き換え
