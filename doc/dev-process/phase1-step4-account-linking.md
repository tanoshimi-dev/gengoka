# Phase 1 Step 4: アカウントリンク機能

## 実装日: 2026-02-17

## 概要
ユーザーが手動でソーシャルアカウント連携を管理（一覧表示・追加・解除）できる機能を Backend / iOS / Android に追加。

## Backend

### 新規モデル (`models/mod.rs`)
- `LinkedSocialAccount` — 連携済みアカウント情報（provider, email, name, linked_at）
- `LinkAccountRequest` — 連携追加リクエスト（provider, id_token, access_token）

### 新規ハンドラー (`handlers/social_auth.rs`)
| Handler | Method | Path | 説明 |
|---------|--------|------|------|
| `get_linked_accounts` | GET | `/api/v1/users/me/social-accounts` | 連携一覧取得 |
| `link_account` | POST | `/api/v1/users/me/social-accounts` | 連携追加 |
| `unlink_account` | DELETE | `/api/v1/users/me/social-accounts/{provider}` | 連携解除 |

### 安全チェック（解除時）
- パスワード設定済み OR 他ソーシャルアカウントが1つ以上残っている場合のみ解除可能
- 最後の認証手段の削除を防止（400エラー）

### ルート登録 (`routes/mod.rs`)
- `/users/me/stats` の直後、`/users/{id}` の前に配置（パスパラメータ競合回避）

## iOS

### 変更ファイル
- `APIEndpoints.swift` — `.linkedAccounts`, `.linkAccount`, `.unlinkAccount(provider:)` 追加
- `AuthModels.swift` — `LinkedSocialAccount`, `LinkAccountRequest` モデル追加
- `MyProfileView.swift` — Settings セクション先頭に「アカウント連携」NavigationLink 追加

### 新規ファイル
- `Views/Profile/LinkedAccountsView.swift` — `LinkedAccountsViewModel`（@Observable）+ `LinkedAccountsView`
  - 全3プロバイダー（Google, Apple, LINE）を一覧表示
  - 既存の `AppleSignInDelegate` を再利用
  - 解除時の確認ダイアログ付き

## Android

### 新規ファイル
- `data/dto/LinkedAccountDto.kt` — `LinkedSocialAccountDto`, `LinkAccountRequestDto`
- `presentation/screens/linkedaccounts/LinkedAccountsViewModel.kt` — ViewModel + UiState
- `presentation/screens/linkedaccounts/LinkedAccountsScreen.kt` — Composable UI（Google + LINE の2行）

### 変更ファイル
- `data/api/GengokApi.kt` — 3エンドポイント追加
- `domain/model/User.kt` — `LinkedSocialAccount` ドメインモデル追加
- `data/mapper/DtoMappers.kt` — `LinkedSocialAccountDto.toDomain()` 追加
- `domain/repository/AuthRepository.kt` — 3メソッド追加
- `data/repository/AuthRepositoryImpl.kt` — 3メソッド実装
- `presentation/navigation/Screen.kt` — `LinkedAccounts` route 追加
- `presentation/navigation/NavGraph.kt` — composable 追加、MyProfileScreen へのコールバック追加
- `presentation/screens/myprofile/MyProfileScreen.kt` — `onLinkedAccountsClick` パラメータ、Settings に「アカウント連携」行追加
