# Gengoka - Feature Overview

## App概要

**Gengoka（言語化）** は、AIを活用した言語表現力トレーニングアプリ。
ユーザーは制限文字数内でお題に回答し、AIからフィードバックを受けることで「言葉にする力」を磨く。

---

## 現在実装済み機能

### 1. 認証システム
| 機能 | Backend | iOS | Android |
|------|---------|-----|---------|
| メール/パスワード登録 | o | o | o |
| メール/パスワードログイン | o | o | o |
| JWTトークン認証 | o | o | o |
| リフレッシュトークン | o | o | o |
| ログアウト | o | o | o |

- **Backend**: Argon2パスワードハッシュ、JWT(30分TTL)、リフレッシュトークン(90日TTL、ローテーション)
- **iOS**: UserDefaultsにトークン保存、@Observableによるリアクティブ状態管理
- **Android**: DataStoreにトークン保存、OkHttp Interceptorによる自動トークン付与

### 2. チャレンジシステム
| カテゴリ | 制限文字数 |
|----------|-----------|
| 状況描写 | 30文字 |
| 要約力 | 50文字 |
| 感性の言語化 | 30文字 |
| 言い換え | 20文字 |
| 概念説明 | 50文字 |

- デイリーチャレンジ（cronスケジューラによる自動生成）
- カテゴリ別チャレンジ一覧
- Gemini APIによるチャレンジ自動生成

### 3. 回答・AIフィードバック
- 文字数制限付き回答投稿
- Gemini AIによるフィードバック（スコア、良い点、改善点、模範回答）
- 回答の編集・削除

### 4. ソーシャル機能
- **タイムライン/フィード**: フィルター付き（全体、フォロー中、人気）
- **いいね**: 回答への評価
- **コメント**: 回答へのフィードバック
- **フォロー/フォロワー**: ユーザー間の関係
- **ランキング**: デイリー、ウィークリー、オールタイム
- **トレンド**: 人気回答の表示

### 5. ユーザー管理
- プロフィール（名前、アバター、自己紹介）
- 学習統計（総チャレンジ数、デイリー完了数、ストリーク、平均スコア）
- マイページ

### 6. 管理パネル（Admin Web）
- Askama テンプレート + Tailwind CSS
- 管理者認証（セッションベース + TOTP 2FA）
- カテゴリ管理（CRUD）
- チャレンジ管理（CRUD + 一括生成）
- ユーザー管理（一覧、詳細、停止/再開）
- コンテンツモデレーション（回答、コメント）
- システム設定（Gemini API、スケジューラ）
- 監査ログ
- ページネーション

---

## Tech Stack

| レイヤー | 技術 |
|---------|------|
| Backend | Rust, Actix-web 4.9, SQLx, PostgreSQL |
| iOS | Swift, SwiftUI, @Observable, URLSession |
| Android | Kotlin, Jetpack Compose, Hilt, Retrofit, OkHttp |
| Admin Web | Askama, Tailwind CSS (Backend一体型) |
| AI | Google Gemini API |
| 認証 | JWT, Argon2, SHA-256 |
| スケジューラ | tokio-cron-scheduler |

---

## アーキテクチャ

### Backend
```
src/
├── main.rs              # エントリポイント、サーバー設定
├── config/              # 環境変数、設定
├── db/                  # マイグレーション、DB接続
├── handlers/            # APIハンドラー
│   ├── auth.rs          # 認証エンドポイント
│   ├── answer.rs        # 回答エンドポイント
│   ├── challenge.rs     # チャレンジエンドポイント
│   ├── user.rs          # ユーザーエンドポイント
│   ├── ranking.rs       # ランキングエンドポイント
│   └── admin/           # 管理パネルハンドラー
├── middleware/           # 認証ミドルウェア
├── models/              # データモデル
├── routes/              # ルーティング定義
├── services/            # ビジネスロジック
│   ├── gemini/          # Gemini APIクライアント
│   └── scheduler/       # チャレンジスケジューラ
└── templates/           # Askama HTMLテンプレート
```

### iOS (MVVM)
```
Gengoka/
├── Services/            # APIClient, AuthService
├── Models/              # データモデル
├── ViewModels/          # 状態管理
├── Views/               # SwiftUI画面
│   ├── Home/            # ホーム、認証画面
│   ├── Challenge/       # チャレンジ画面
│   ├── Feed/            # フィード画面
│   ├── Ranking/         # ランキング画面
│   └── MyPage/          # マイページ画面
└── Resources/           # 定数、カラー
```

### Android (Clean Architecture + MVVM)
```
app/src/main/kotlin/app/dev/gengoka/
├── core/                # ネットワーク、ユーティリティ
│   ├── network/         # TokenManager, AuthInterceptor
│   └── util/            # Resource, safeApiCall
├── data/                # データ層
│   ├── api/             # Retrofitインターフェース
│   ├── dto/             # データ転送オブジェクト
│   └── repository/      # リポジトリ実装
├── domain/              # ドメイン層
│   ├── model/           # ドメインモデル
│   └── repository/      # リポジトリインターフェース
├── presentation/        # プレゼンテーション層
│   ├── screens/         # Compose画面
│   ├── components/      # 共通UIコンポーネント
│   └── theme/           # テーマ、カラー
└── di/                  # Hilt DI モジュール
```

---

## API エンドポイント一覧

### 認証 (`/api/v1/auth`)
| Method | Path | 説明 |
|--------|------|------|
| POST | /auth/register | ユーザー登録 |
| POST | /auth/login | ログイン |
| POST | /auth/refresh | トークンリフレッシュ |
| POST | /auth/logout | ログアウト |

### カテゴリ (`/api/v1/categories`)
| Method | Path | 説明 |
|--------|------|------|
| GET | /categories | カテゴリ一覧 |
| GET | /categories/{id} | カテゴリ詳細 |
| GET | /categories/{id}/challenges | カテゴリ別チャレンジ |

### チャレンジ (`/api/v1/challenges`)
| Method | Path | 説明 |
|--------|------|------|
| GET | /challenges | チャレンジ一覧 |
| GET | /challenges/daily | デイリーチャレンジ |
| GET | /challenges/{id} | チャレンジ詳細 |
| GET | /challenges/{id}/answers | チャレンジの回答一覧 |
| POST | /challenges/{id}/answers | 回答投稿 |

### 回答 (`/api/v1/answers`)
| Method | Path | 説明 |
|--------|------|------|
| GET | /answers/{id} | 回答詳細 |
| PUT | /answers/{id} | 回答更新 |
| DELETE | /answers/{id} | 回答削除 |
| POST | /answers/{id}/like | いいね |
| DELETE | /answers/{id}/like | いいね取消 |
| GET | /answers/{id}/comments | コメント一覧 |
| POST | /answers/{id}/comments | コメント投稿 |

### ユーザー (`/api/v1/users`)
| Method | Path | 説明 |
|--------|------|------|
| GET | /users/me | 現在のユーザー |
| GET | /users/me/stats | 学習統計 |
| GET | /users/{id} | ユーザー詳細 |
| PUT | /users/{id} | プロフィール更新 |
| GET | /users/{id}/answers | ユーザーの回答一覧 |
| POST | /users/{id}/follow | フォロー |
| DELETE | /users/{id}/follow | アンフォロー |
| GET | /users/{id}/followers | フォロワー一覧 |
| GET | /users/{id}/following | フォロー中一覧 |

### フィード・ランキング
| Method | Path | 説明 |
|--------|------|------|
| GET | /feed | パーソナライズフィード |
| GET | /trending | トレンド |
| GET | /rankings/daily | デイリーランキング |
| GET | /rankings/weekly | ウィークリーランキング |
| GET | /rankings/all-time | オールタイムランキング |

---

## データベーススキーマ

### コアテーブル
- **users** - ユーザー情報（email, name, avatar, bio, password_hash, total_likes, status）
- **categories** - カテゴリ（name, description, icon, color, char_limit, sort_order）
- **challenges** - チャレンジ（category_id, title, description, char_limit, release_date）
- **answers** - 回答（challenge_id, user_id, content, score, ai_feedback[JSONB]）
- **comments** - コメント（answer_id, user_id, content, status）
- **likes** - いいね（answer_id, user_id / UNIQUE）
- **follows** - フォロー（follower_id, following_id / UNIQUE）

### 認証テーブル
- **refresh_tokens** - リフレッシュトークン（user_id, token_hash, device_info, expires_at）

### 管理テーブル
- **admin_users** - 管理者（email, password_hash, role, totp_secret, totp_enabled）
- **admin_audit_logs** - 監査ログ（admin_user_id, action, entity_type, details[JSONB]）
- **admin_2fa_backup_codes** - 2FAバックアップコード
- **system_config** - システム設定（key, value[JSONB]）

---

## 現在の課題・未実装

| 項目 | 状態 |
|------|------|
| ソーシャル認証（Google, Apple, LINE） | 未実装 |
| ユニットテスト | 未実装（テストファイルなし） |
| E2Eテスト | 未実装 |
| パスワードリセット | TODO |
| プッシュ通知 | spec04作成済み、未実装 |
| メール認証 | 未実装 |
| iOS Keychain移行（UserDefaults→Keychain） | 未実装 |
| Web フロントエンド（Next.js） | プレースホルダーのみ |
| HTTPS / 証明書ピニング | 本番環境向け未対応 |
| レート制限 | 未実装 |
