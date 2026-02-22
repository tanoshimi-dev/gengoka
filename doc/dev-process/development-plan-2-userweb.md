# Gengoka - User Web 開発計画

## 概要

モバイルアプリ（iOS / Android）で提供している全機能をWebブラウザでも利用可能にする。
既存の Backend API（`/api/v1/*`）をそのまま活用し、新規バックエンド開発は最小限に抑える。

---

## 技術スタック

| カテゴリ | 技術 | 理由 |
|---------|------|------|
| フレームワーク | Next.js 15 (App Router) | SSR/SSG、SEO対応、React Server Components |
| 言語 | TypeScript | 型安全性、DX向上 |
| スタイリング | Tailwind CSS 4 | ユーティリティファースト、高速開発 |
| UIコンポーネント | shadcn/ui | カスタマイズ性、Tailwind互換、コピペ方式で軽量 |
| 状態管理 | Zustand | 軽量、シンプル（認証・グローバル状態） |
| データフェッチ | TanStack Query (React Query) | キャッシュ、楽観的更新、無限スクロール |
| フォーム | React Hook Form + Zod | バリデーション、型安全 |
| 認証 | NextAuth.js (Auth.js v5) | Google/Apple/LINE OAuth、セッション管理 |
| アニメーション | Framer Motion | ページ遷移、マイクロインタラクション |
| テスト | Vitest + Playwright | ユニットテスト + E2Eテスト |
| リンター | ESLint + Prettier | コード品質 |
| パッケージマネージャ | pnpm | 高速、ディスク効率 |
| デプロイ | VPS (Docker + Traefik) | 既存Backend と同一サーバー、Cloudflare SSL、完全制御 |

---

## プロジェクト構成

```
sys/frontend/user/web/
├── public/
│   ├── favicon.ico
│   ├── og-image.png
│   └── icons/
├── src/
│   ├── app/                          # Next.js App Router
│   │   ├── layout.tsx                # ルートレイアウト
│   │   ├── page.tsx                  # LP（未ログイン）/ ホーム（ログイン済）
│   │   ├── globals.css               # グローバルスタイル
│   │   ├── (auth)/                   # 認証グループ
│   │   │   ├── login/page.tsx
│   │   │   ├── register/page.tsx
│   │   │   └── layout.tsx
│   │   ├── (main)/                   # メインアプリグループ
│   │   │   ├── layout.tsx            # ヘッダー + サイドバー
│   │   │   ├── home/page.tsx         # ホーム（デイリーチャレンジ）
│   │   │   ├── challenges/
│   │   │   │   ├── [id]/page.tsx     # チャレンジ詳細・回答
│   │   │   │   └── [id]/result/page.tsx  # 回答結果
│   │   │   ├── categories/
│   │   │   │   └── [id]/page.tsx     # カテゴリ別チャレンジ一覧
│   │   │   ├── feed/page.tsx         # タイムライン
│   │   │   ├── profile/
│   │   │   │   └── [id]/page.tsx     # 他ユーザープロフィール
│   │   │   ├── mypage/page.tsx       # マイページ
│   │   │   ├── mypage/settings/page.tsx  # 設定
│   │   │   └── rankings/page.tsx     # ランキング
│   │   └── api/                      # API Routes（BFF）
│   │       └── auth/
│   │           └── [...nextauth]/route.ts
│   ├── components/
│   │   ├── ui/                       # shadcn/ui ベースコンポーネント
│   │   ├── layout/
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   ├── BottomNav.tsx         # モバイルWeb用
│   │   │   └── Footer.tsx
│   │   ├── auth/
│   │   │   ├── LoginForm.tsx
│   │   │   ├── RegisterForm.tsx
│   │   │   └── SocialLoginButtons.tsx
│   │   ├── challenge/
│   │   │   ├── ChallengeCard.tsx
│   │   │   ├── DailyChallengeList.tsx
│   │   │   ├── AnswerForm.tsx
│   │   │   ├── ResultDisplay.tsx
│   │   │   └── ScoreCircle.tsx
│   │   ├── feed/
│   │   │   ├── FeedCard.tsx
│   │   │   ├── FeedFilter.tsx
│   │   │   ├── CommentSection.tsx
│   │   │   └── LikeButton.tsx
│   │   ├── profile/
│   │   │   ├── ProfileHeader.tsx
│   │   │   ├── StatsGrid.tsx
│   │   │   ├── AnswerList.tsx
│   │   │   └── FollowButton.tsx
│   │   ├── category/
│   │   │   └── CategoryGrid.tsx
│   │   └── common/
│   │       ├── CharacterCounter.tsx
│   │       ├── GradientButton.tsx
│   │       ├── UserAvatar.tsx
│   │       ├── LoadingSpinner.tsx
│   │       └── ErrorMessage.tsx
│   ├── lib/
│   │   ├── api/
│   │   │   ├── client.ts             # APIクライアント（fetch wrapper）
│   │   │   ├── endpoints.ts          # エンドポイント定義
│   │   │   └── types.ts              # APIレスポンス型
│   │   ├── auth/
│   │   │   ├── config.ts             # NextAuth設定
│   │   │   └── providers.ts          # OAuth プロバイダー設定
│   │   └── utils/
│   │       ├── date.ts               # 日付ユーティリティ
│   │       ├── validation.ts         # Zodスキーマ
│   │       └── constants.ts          # 定数
│   ├── hooks/
│   │   ├── useAuth.ts                # 認証フック
│   │   ├── useChallenges.ts          # チャレンジ系クエリ
│   │   ├── useFeed.ts                # フィード系クエリ（無限スクロール）
│   │   ├── useProfile.ts             # プロフィール系クエリ
│   │   └── useRankings.ts            # ランキング系クエリ
│   ├── stores/
│   │   └── auth-store.ts             # Zustand認証ストア
│   └── types/
│       ├── user.ts
│       ├── challenge.ts
│       ├── answer.ts
│       ├── category.ts
│       ├── comment.ts
│       └── feed.ts
├── tests/
│   ├── unit/                         # Vitestユニットテスト
│   └── e2e/                          # Playwrightテスト
├── next.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── package.json
├── Dockerfile                        # マルチステージビルド
├── .dockerignore
├── .env.local                        # ローカル環境変数
└── .env.example
```

---

## 利用する既存 Backend API

Web フロントエンドは既存の API をそのまま利用する。Backend 側の変更は最小限。

### Backend 側で必要な変更

| 変更 | 内容 | 理由 |
|------|------|------|
| CORS設定更新 | Web ドメイン追加 | 現在は全オリジン許可だが、本番ではドメイン制限 |
| OAuth リダイレクトURI追加 | Web用コールバックURL | Google/Apple/LINE の各プロバイダーコンソールに追加 |
| Cookie対応（任意） | SameSite, Secure属性 | トークンをCookieで管理する場合 |

### Web 認証フロー

モバイルアプリではSDKベースの認証だが、Webでは標準OAuth 2.0フローを使用する。

```
[ブラウザ]
    │
    ├─ メール/パスワード認証
    │   └─ POST /api/v1/auth/login (直接APIコール)
    │       └─ JWT(access_token + refresh_token) を取得
    │           └─ メモリ + httpOnly Cookie に保存
    │
    ├─ Google OAuth (NextAuth.js)
    │   ├─ Google認証画面にリダイレクト
    │   ├─ コールバックでIDトークン取得
    │   └─ POST /api/v1/auth/social { provider: "google", id_token: "..." }
    │       └─ JWT取得
    │
    ├─ Apple OAuth (NextAuth.js)
    │   ├─ Apple認証画面にリダイレクト
    │   ├─ コールバックでIDトークン取得
    │   └─ POST /api/v1/auth/social { provider: "apple", id_token: "..." }
    │       └─ JWT取得
    │
    └─ LINE OAuth (NextAuth.js)
        ├─ LINE認証画面にリダイレクト
        ├─ コールバックでアクセストークン取得
        └─ POST /api/v1/auth/social { provider: "line", access_token: "..." }
            └─ JWT取得
```

---

## Phase 1: プロジェクト基盤 + 認証

### 1.1 概要

Next.js プロジェクトの初期構築、認証機能の実装を行う。

### 1.2 開発ステップ

```
Step 1: プロジェクト初期構築
         ├─ Next.js 15 + TypeScript プロジェクト作成
         ├─ Tailwind CSS 4 + shadcn/ui セットアップ
         ├─ ESLint + Prettier 設定
         ├─ ディレクトリ構造作成
         ├─ 環境変数設定（.env.local, .env.example）
         └─ APIクライアント（fetch wrapper + 型定義）作成
              │
Step 2: 認証機能実装
         ├─ NextAuth.js (Auth.js v5) セットアップ
         │   ├─ Google OAuth プロバイダー
         │   ├─ Apple OAuth プロバイダー
         │   └─ LINE OAuth プロバイダー
         ├─ メール/パスワード ログイン画面
         ├─ メール/パスワード 新規登録画面
         ├─ ソーシャルログインボタン
         ├─ JWT トークン管理（Zustand ストア）
         ├─ 認証ミドルウェア（Next.js middleware.ts）
         ├─ 自動トークンリフレッシュ
         └─ ログアウト
              │
Step 3: 共通レイアウト・デザインシステム
         ├─ ルートレイアウト（フォント、メタデータ）
         ├─ ヘッダー（ロゴ、ナビ、ユーザーメニュー）
         ├─ サイドバー（PC）/ BottomNav（モバイルWeb）
         ├─ レスポンシブ対応（PC / タブレット / モバイル）
         ├─ テーマ（カラーパレット：モバイルアプリと統一）
         │   ├─ Primary: #667eea → #764ba2 グラデーション
         │   ├─ カテゴリカラー（5色）
         │   └─ セマンティックカラー（success / warning / error）
         └─ 共通コンポーネント
             ├─ GradientButton
             ├─ UserAvatar
             ├─ LoadingSpinner
             └─ ErrorMessage
```

### 1.3 画面仕様

#### ログイン画面 (`/login`)

```
┌─────────────────────────────────┐
│           Gengoka ロゴ           │
│                                 │
│  ┌───────────────────────────┐  │
│  │ メールアドレス             │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ パスワード                 │  │
│  └───────────────────────────┘  │
│                                 │
│  [======= ログイン =======]     │
│                                 │
│  ─────── または ───────          │
│                                 │
│  [G] Googleでログイン           │
│  [🍎] Appleでログイン           │
│  [LINE] LINEでログイン          │
│                                 │
│  アカウントをお持ちでない方 →    │
│         新規登録                 │
└─────────────────────────────────┘
```

#### 新規登録画面 (`/register`)

```
┌─────────────────────────────────┐
│           Gengoka ロゴ           │
│                                 │
│  ┌───────────────────────────┐  │
│  │ ユーザー名                 │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ 表示名                     │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ メールアドレス             │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ パスワード                 │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ パスワード確認             │  │
│  └───────────────────────────┘  │
│                                 │
│  [======= 新規登録 =======]     │
│                                 │
│  ─────── または ───────          │
│                                 │
│  [G] Googleで登録               │
│  [🍎] Appleで登録               │
│  [LINE] LINEで登録              │
│                                 │
│  すでにアカウントをお持ちの方 →  │
│         ログイン                 │
└─────────────────────────────────┘
```

---

## Phase 2: ホーム + チャレンジ機能

### 2.1 概要

ホーム画面、カテゴリ一覧、デイリーチャレンジ、チャレンジ回答、結果表示を実装する。

### 2.2 開発ステップ

```
Step 1: ホーム画面
         ├─ ユーザー統計表示（ストリーク、本日完了数、平均スコア、累計）
         ├─ デイリーチャレンジ一覧（最大5件、完了状態表示）
         ├─ カテゴリグリッド（アイコン、カラー、チャレンジ数）
         └─ useChallenges / useProfile カスタムフック
              │
Step 2: チャレンジ画面
         ├─ チャレンジ詳細表示（カテゴリ、タイトル、説明、文字数制限）
         ├─ 回答入力フォーム
         │   ├─ CharacterCounter（リアルタイム文字数表示）
         │   ├─ 文字数バリデーション（min/max）
         │   └─ 回答済みの場合は既存回答を表示
         ├─ 回答送信（POST /api/v1/challenges/{id}/answers）
         └─ 重複回答エラーハンドリング（409）
              │
Step 3: 結果画面
         ├─ スコア表示（円形プログレス、アニメーション）
         ├─ AIフィードバック表示
         │   ├─ 良い点（緑カード）
         │   ├─ 改善点（オレンジカード）
         │   └─ 例文（青カード）
         ├─ 元の回答表示
         └─ 「別の問題」「もう一度」ボタン
              │
Step 4: カテゴリ別チャレンジ一覧
         ├─ カテゴリ詳細（名前、説明、チャレンジ数）
         ├─ チャレンジ一覧（ページネーション）
         └─ チャレンジカードからチャレンジ画面への遷移
```

### 2.3 画面仕様

#### ホーム画面 (`/home`)

```
PC レイアウト:
┌──────────────────────────────────────────────┐
│  [サイドバー]  │  こんにちは、{name}さん！     │
│               │                               │
│  ホーム ●     │  ┌─────────────────────────┐   │
│  タイムライン  │  │ 📊 ストリーク: 5日        │   │
│  マイページ   │  │    本日: 2/5  平均: 82点  │   │
│  ランキング   │  └─────────────────────────┘   │
│               │                               │
│               │  📝 今日のチャレンジ            │
│               │  ┌─────┐ ┌─────┐ ┌─────┐     │
│               │  │ お題1 │ │ お題2 │ │ お題3 │     │
│               │  │ ✅   │ │      │ │      │     │
│               │  └─────┘ └─────┘ └─────┘     │
│               │                               │
│               │  📂 カテゴリ                    │
│               │  ┌──────┐ ┌──────┐            │
│               │  │状況描写│ │要約  │            │
│               │  │ 12問  │ │ 8問  │            │
│               │  └──────┘ └──────┘            │
│               │  ┌──────┐ ┌──────┐            │
│               │  │感情表現│ │言換え│            │
│               │  │ 10問  │ │ 6問  │            │
│               │  └──────┘ └──────┘            │
└──────────────────────────────────────────────┘
```

#### チャレンジ画面 (`/challenges/[id]`)

```
┌─────────────────────────────────┐
│  ← 戻る                         │
│                                 │
│  [状況描写]  お題タイトル         │
│                                 │
│  お題の説明文がここに表示される。  │
│                                 │
│  ┌───────────────────────────┐  │
│  │                           │  │
│  │  あなたの回答を入力...      │  │
│  │                           │  │
│  └───────────────────────────┘  │
│                    12 / 30 文字  │
│                                 │
│  [======= 回答する =======]     │
└─────────────────────────────────┘
```

#### 結果画面 (`/challenges/[id]/result`)

```
┌─────────────────────────────────┐
│                                 │
│         ┌─────────┐             │
│         │  ○ 85   │             │
│         │  /100   │             │
│         └─────────┘             │
│       素晴らしい回答です！        │
│                                 │
│  あなたの回答:                   │
│  「ここに回答テキスト」           │
│                                 │
│  ┌─ 💚 良い点 ────────────────┐ │
│  │ フィードバック内容           │ │
│  └────────────────────────────┘ │
│  ┌─ 🟠 改善点 ────────────────┐ │
│  │ フィードバック内容           │ │
│  └────────────────────────────┘ │
│  ┌─ 🔵 例文 ──────────────────┐ │
│  │ 例文テキスト                │ │
│  └────────────────────────────┘ │
│                                 │
│  [別の問題へ]  [もう一度挑戦]    │
└─────────────────────────────────┘
```

---

## Phase 3: フィード + ソーシャル機能

### 3.1 概要

タイムライン（フィード）、いいね、コメント、ユーザーフォロー機能を実装する。

### 3.2 開発ステップ

```
Step 1: フィード画面
         ├─ フィードカード（ユーザー、回答、スコア、いいね数、コメント数）
         ├─ フィルタータブ（すべて / フォロー中 / カテゴリ別）
         ├─ 無限スクロール（TanStack Query useInfiniteQuery）
         ├─ プルリフレッシュ（モバイルWeb対応）
         └─ useFeed カスタムフック
              │
Step 2: いいね・コメント
         ├─ いいねボタン（楽観的更新）
         ├─ コメントセクション（モーダル or ドロワー）
         │   ├─ コメント一覧（ページネーション）
         │   ├─ コメント投稿フォーム
         │   └─ コメント削除（自分のコメントのみ）
         └─ アニメーション（いいねハート等）
              │
Step 3: ユーザープロフィール画面
         ├─ プロフィールヘッダー（アバター、名前、bio、レベル）
         ├─ 統計グリッド（回答数、フォロワー、フォロー中）
         ├─ フォロー/アンフォローボタン
         ├─ ユーザーの回答一覧（タブ切り替え）
         └─ useProfile カスタムフック
              │
Step 4: マイページ
         ├─ プロフィール情報表示・編集
         ├─ 学習統計（ストリーク、ベスト、本日、平均、累計）
         ├─ ソーシャル統計（フォロワー、フォロー中、回答数）
         ├─ アカウント連携管理（Google / Apple / LINE）
         └─ 設定メニュー（通知、プライバシー、ログアウト）
```

### 3.3 画面仕様

#### フィード画面 (`/feed`)

```
┌──────────────────────────────────────────────┐
│  [サイドバー]  │  タイムライン                  │
│               │                               │
│               │  [すべて] [フォロー中] [カテゴリ]│
│               │                               │
│               │  ┌────────────────────────┐    │
│               │  │ 👤 ユーザー名  2時間前   │    │
│               │  │ お題: 状況描写のタイトル  │    │
│               │  │                        │    │
│               │  │ 「回答テキスト」         │    │
│               │  │                        │    │
│               │  │ スコア: 85点            │    │
│               │  │ ❤️ 12  💬 3            │    │
│               │  └────────────────────────┘    │
│               │                               │
│               │  ┌────────────────────────┐    │
│               │  │ 👤 別のユーザー  5時間前 │    │
│               │  │ ...                    │    │
│               │  └────────────────────────┘    │
│               │                               │
│               │  [もっと読み込む...]            │
└──────────────────────────────────────────────┘
```

#### プロフィール画面 (`/profile/[id]`)

```
┌─────────────────────────────────┐
│  ← 戻る                         │
│                                 │
│  ┌──┐  ユーザー表示名            │
│  │👤│  @username                 │
│  └──┘  自己紹介テキスト...        │
│                                 │
│  回答: 42  フォロワー: 128       │
│  フォロー中: 56                  │
│                                 │
│  [フォローする]                   │
│                                 │
│  ─── 回答一覧 ───                │
│  ┌────────────────────────┐     │
│  │ 回答1...                │     │
│  └────────────────────────┘     │
│  ┌────────────────────────┐     │
│  │ 回答2...                │     │
│  └────────────────────────┘     │
└─────────────────────────────────┘
```

---

## Phase 4: ランキング + Web固有機能

### 4.1 概要

ランキング表示、Web固有のSEO対応、OGP（シェアカード）、PWA対応を実装する。

### 4.2 開発ステップ

```
Step 1: ランキング画面
         ├─ デイリー / ウィークリー / 累計 タブ切り替え
         ├─ ランキングテーブル（順位、アバター、名前、スコア）
         ├─ 自分の順位ハイライト
         └─ useRankings カスタムフック
              │
Step 2: SEO + OGP対応
         ├─ メタデータ設定（各ページ個別）
         ├─ OGP画像自動生成（チャレンジ共有用）
         │   └─ /api/og?challengeId=xxx → 動的OGP画像
         ├─ sitemap.xml 自動生成
         ├─ robots.txt
         └─ JSON-LD 構造化データ
              │
Step 3: レスポンシブ最適化
         ├─ モバイルWeb最適化（タッチ操作、BottomNav）
         ├─ タブレット対応（2カラムレイアウト）
         ├─ PC対応（サイドバー + メインコンテンツ）
         └─ ダークモード対応（任意）
              │
Step 4: パフォーマンス最適化
         ├─ 画像最適化（next/image）
         ├─ コード分割（dynamic import）
         ├─ キャッシュ戦略（TanStack Query staleTime設定）
         └─ Core Web Vitals最適化
```

### 4.3 画面仕様

#### ランキング画面 (`/rankings`)

```
┌──────────────────────────────────────────────┐
│  [サイドバー]  │  ランキング                    │
│               │                               │
│               │  [デイリー] [ウィークリー] [累計] │
│               │                               │
│               │  🥇 1. ユーザーA    950pt       │
│               │  🥈 2. ユーザーB    920pt       │
│               │  🥉 3. ユーザーC    890pt       │
│               │     4. ユーザーD    850pt       │
│               │     5. ユーザーE    810pt       │
│               │     ...                       │
│               │  ─────────────────────         │
│               │  ⭐ 42. あなた      520pt       │
└──────────────────────────────────────────────┘
```

---

## Phase 5: テスト + デプロイ

### 5.1 概要

テスト整備、CI/CD パイプライン構築、本番デプロイを行う。

### 5.2 開発ステップ

```
Step 1: ユニットテスト
         ├─ APIクライアントテスト
         ├─ カスタムフックテスト（TanStack Query）
         ├─ Zodバリデーションテスト
         ├─ ユーティリティ関数テスト
         └─ コンポーネントテスト（主要UI）
              │
Step 2: E2Eテスト
         ├─ 認証フロー（ログイン → ホーム → ログアウト）
         ├─ チャレンジフロー（お題選択 → 回答 → 結果確認）
         ├─ フィードフロー（閲覧 → いいね → コメント）
         ├─ プロフィールフロー（閲覧 → フォロー）
         └─ レスポンシブテスト（PC / モバイル）
              │
Step 3: CI/CD + VPSデプロイ
         ├─ Dockerfile 作成（マルチステージビルド）
         │   ├─ Stage 1: deps（依存関係インストール）
         │   ├─ Stage 2: builder（next build）
         │   └─ Stage 3: runner（node:alpine で本番実行）
         ├─ docker-compose.prod.yml 更新（web サービス追加）
         │   ├─ Traefik ラベル設定（gengoka.app ルーティング）
         │   ├─ gengoka-network + traefik-network 接続
         │   └─ Cloudflare SSL（既存 certresolver 利用）
         ├─ GitHub Actions ワークフロー
         │   ├─ lint + type-check
         │   ├─ ユニットテスト + E2Eテスト
         │   └─ VPSへの自動デプロイ（SSH + Docker）
         └─ Cloudflare DNS設定
              │
Step 4: 最終検証
         ├─ クロスブラウザテスト（Chrome, Safari, Firefox, Edge）
         ├─ パフォーマンス監査（Lighthouse）
         ├─ アクセシビリティ監査
         └─ 本番環境動作確認
```

---

## Phase 6: モバイルレスポンシブ対応

### 6.1 概要

現状の Web UI はデスクトップ向けレイアウトのみで、モバイル端末ではレイアウトが崩れる。
全画面をモバイルファーストで再調整し、スマートフォン・タブレットで快適に利用できるようにする。

### 6.2 開発ステップ

```
Step 1: レイアウト・ナビゲーション修正
         ├─ BottomNav 実装（モバイル時にサイドバーを非表示→BottomNavに切替）
         ├─ ヘッダーのモバイル対応（ハンバーガーメニュー or 簡略化）
         ├─ サイドバーをドロワー化（タブレット以下で開閉式）
         └─ ブレークポイント統一（sm:640px / md:768px / lg:1024px）
              │
Step 2: 各画面のモバイル対応
         ├─ ログイン・新規登録画面（フォーム幅・余白調整）
         ├─ ホーム画面（統計カード・チャレンジカードの1カラム化）
         ├─ チャレンジ回答画面（入力フォームのモバイル最適化）
         ├─ 結果画面（スコア・フィードバックカードのスタック表示）
         ├─ フィード画面（カード幅100%、余白調整）
         ├─ プロフィール・マイページ（統計グリッドの折り返し）
         └─ ランキング画面（テーブルのスクロール or カード表示）
              │
Step 3: タッチ操作・UX改善
         ├─ タップターゲットサイズ確保（最小44px）
         ├─ スワイプ操作対応（フィードのプルリフレッシュ等）
         ├─ フォント・余白のモバイル向け調整
         └─ モーダル・ドロワーのモバイル対応
              │
Step 4: 検証
         ├─ 実機テスト（iPhone / Android）
         ├─ Chrome DevTools レスポンシブモードで全画面確認
         └─ Lighthouse モバイルスコア改善
```

---

## レスポンシブ対応方針

| ブレークポイント | レイアウト | ナビゲーション |
|---------------|-----------|-------------|
| `< 640px` (モバイル) | シングルカラム | BottomNav |
| `640px - 1024px` (タブレット) | 2カラム | ヘッダーナビ |
| `> 1024px` (PC) | サイドバー + メインコンテンツ | サイドバー |

---

## API クライアント設計

```typescript
// lib/api/client.ts（概要）

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api/v1";

async function apiClient<T>(
  endpoint: string,
  options?: {
    method?: "GET" | "POST" | "PUT" | "DELETE";
    body?: unknown;
    token?: string;
  }
): Promise<T> {
  const headers: HeadersInit = { "Content-Type": "application/json" };
  if (options?.token) {
    headers["Authorization"] = `Bearer ${options.token}`;
  }

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    method: options?.method || "GET",
    headers,
    body: options?.body ? JSON.stringify(options.body) : undefined,
  });

  if (!response.ok) {
    // エラーハンドリング
  }

  return response.json();
}
```

---

## モバイルアプリとの機能対応表

| 機能 | iOS | Android | Web |
|------|-----|---------|-----|
| メール/パスワード認証 | ✅ | ✅ | Phase 1 |
| Google ソーシャルログイン | ✅ | ✅ | Phase 1 |
| Apple ソーシャルログイン | ✅ | - | Phase 1 |
| LINE ソーシャルログイン | ✅ | ✅ | Phase 1 |
| ホーム画面（統計・デイリー） | ✅ | ✅ | Phase 2 |
| カテゴリ一覧 | ✅ | ✅ | Phase 2 |
| チャレンジ回答 | ✅ | ✅ | Phase 2 |
| AI スコア・フィードバック | ✅ | ✅ | Phase 2 |
| タイムライン（フィード） | ✅ | ✅ | Phase 3 |
| いいね・コメント | ✅ | ✅ | Phase 3 |
| ユーザープロフィール | ✅ | ✅ | Phase 3 |
| フォロー / アンフォロー | ✅ | ✅ | Phase 3 |
| マイページ・設定 | ✅ | ✅ | Phase 3 |
| アカウント連携管理 | ✅ | ✅ | Phase 3 |
| ランキング | ✅ | ✅ | Phase 4 |
| SEO / OGP | - | - | Phase 4（Web固有） |
| PWA対応 | - | - | Phase 4（Web固有） |

---

## 開発スケジュール目安

```
Phase 1: プロジェクト基盤 + 認証
├── Step 1 (初期構築)           ─── 基盤
├── Step 2 (認証)               ─── 実装
└── Step 3 (レイアウト・デザイン) ─── 実装

Phase 2: ホーム + チャレンジ機能
├── Step 1 (ホーム)             ─── 実装
├── Step 2 (チャレンジ)         ─── 実装
├── Step 3 (結果)               ─── 実装
└── Step 4 (カテゴリ)           ─── 実装

Phase 3: フィード + ソーシャル機能
├── Step 1 (フィード)           ─── 実装
├── Step 2 (いいね・コメント)    ─── 実装
├── Step 3 (プロフィール)       ─── 実装
└── Step 4 (マイページ)         ─── 実装

Phase 4: ランキング + Web固有機能
├── Step 1 (ランキング)         ─── 実装
├── Step 2 (SEO / OGP)         ─── 実装
├── Step 3 (レスポンシブ最適化)  ─── 実装
└── Step 4 (パフォーマンス)     ─── 最適化

Phase 5: テスト + デプロイ
├── Step 1 (ユニットテスト)     ─── QA
├── Step 2 (E2Eテスト)          ─── QA
├── Step 3 (CI/CD + VPSデプロイ) ─── インフラ
└── Step 4 (最終検証)           ─── QA

Phase 6: モバイルレスポンシブ対応
├── Step 1 (レイアウト・ナビゲーション) ─── UI修正
├── Step 2 (各画面のモバイル対応)       ─── UI修正
├── Step 3 (タッチ操作・UX改善)         ─── UX改善
└── Step 4 (検証)                       ─── QA
```

---

## VPS デプロイ構成

### サーバー構成図

既存インフラ（Traefik + Cloudflare）に Next.js コンテナを追加する。

```
[VPS Server]
│
├─ Traefik（リバースプロキシ + SSL終端、Cloudflare certresolver）
│   ├─ gengoka.app          → :3000 (Next.js Web)  ← 既存 frontend を置換
│   ├─ backend.gengoka.app  → :8080 (Backend API)  ← 既存
│   ├─ webmail.gengoka.app  → Roundcube            ← 既存
│   └─ mail.gengoka.app     → Mail Server          ← 既存
│
├─ Docker Compose (docker-compose.prod.yml)
│   ├─ web       (Next.js, port 3000)      ← 新規（既存 frontend を置換）
│   ├─ backend   (Rust/Actix-web, port 8080)  ← 既存
│   ├─ postgres  (PostgreSQL 16)               ← 既存
│   ├─ adminer   (DB管理)                      ← 既存
│   ├─ mail      (docker-mailserver)           ← 既存
│   └─ webmail   (Roundcube)                   ← 既存
│
├─ Networks
│   ├─ gengoka-network  (内部通信)
│   └─ traefik-network  (外部、Traefik連携)
│
└─ Cloudflare DNS（SSL証明書自動管理）
```

### Dockerfile（Next.js マルチステージビルド）

```dockerfile
# Stage 1: 依存関係
FROM node:22-alpine AS deps
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile

# Stage 2: ビルド
FROM node:22-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN corepack enable && pnpm build

# Stage 3: 本番実行
FROM node:22-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production

COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

EXPOSE 3000
CMD ["node", "server.js"]
```

### docker-compose.prod.yml 変更

既存の `frontend`（nginx:alpine 静的サイト）を Next.js SSR コンテナに置換する。

```yaml
  # ============================================
  # Frontend (Next.js SSR) ← nginx:alpine から置換
  # ============================================
  web:
    build:
      context: ./frontend/web
      dockerfile: Dockerfile
    container_name: gengoka-web
    env_file:
      - .env.web
    environment:
      - NEXT_PUBLIC_API_URL=https://backend.gengoka.app/api/v1
      - NEXTAUTH_URL=https://gengoka.app
    restart: unless-stopped
    networks:
      - gengoka-network
      - traefik-network
    depends_on:
      - backend
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.gengoka-web.rule=Host(`gengoka.app`)"
      - "traefik.http.routers.gengoka-web.entrypoints=https"
      - "traefik.http.routers.gengoka-web.tls=true"
      - "traefik.http.routers.gengoka-web.tls.certresolver=cloudflare"
      - "traefik.http.services.gengoka-web.loadbalancer.server.port=3000"
      - "traefik.docker.network=traefik-network"
    deploy:
      resources:
        limits:
          memory: 512M
```

### GitHub Actions デプロイワークフロー

```yaml
# .github/workflows/deploy-web.yml
name: Deploy Web

on:
  push:
    branches: [main]
    paths: ["sys/frontend/user/web/**"]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Deploy to VPS
        uses: appleboy/ssh-action@v1
        with:
          host: ${{ secrets.VPS_HOST }}
          username: ${{ secrets.VPS_USER }}
          key: ${{ secrets.VPS_SSH_KEY }}
          script: |
            cd /path/to/gengoka/sys/backend
            git pull origin main
            docker compose -f docker-compose.prod.yml build web
            docker compose -f docker-compose.prod.yml up -d web
```

---

## 環境変数

### ローカル開発用 (`.env.local`)

```env
# Backend API
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1

# NextAuth.js
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your-secret-key

# Google OAuth (Web用)
GOOGLE_CLIENT_ID=xxx.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=xxx

# Apple OAuth (Web用)
APPLE_CLIENT_ID=app.dev.gengoka.web
APPLE_CLIENT_SECRET=xxx

# LINE OAuth (Web用)
LINE_CLIENT_ID=1234567890
LINE_CLIENT_SECRET=xxx
```

### 本番用 (`sys/backend/.env.web`)

```env
# Backend API（Docker内部通信はコンテナ名、公開URLはTraefik経由）
NEXT_PUBLIC_API_URL=https://backend.gengoka.app/api/v1

# NextAuth.js
NEXTAUTH_URL=https://gengoka.app
NEXTAUTH_SECRET=your-production-secret

# OAuth credentials（本番用）
GOOGLE_CLIENT_ID=xxx
GOOGLE_CLIENT_SECRET=xxx
APPLE_CLIENT_ID=xxx
APPLE_CLIENT_SECRET=xxx
LINE_CLIENT_ID=xxx
LINE_CLIENT_SECRET=xxx
```

---

## 備考

- 既存の Backend API は変更不要（CORS設定とOAuth設定のみ更新）
- モバイルアプリとデザインテーマを統一（カラー、グラデーション、カテゴリカラー）
- 各Phase完了時にクロスブラウザ確認を実施
- SEO / OGP はWeb固有の追加価値として重点対応
- PWA対応はPhase 4で検討（オフライン対応、ホーム画面追加）
- 既存の `frontend`（nginx:alpine 静的配信）を Next.js SSR コンテナに置換
- 既存の `docker-compose.prod.yml` に `web` サービスとして追加
- Traefik ラベルで `gengoka.app` → `:3000` にルーティング（既存パターン踏襲）
- SSL は Cloudflare certresolver で自動管理（既存と同一）
- Next.js は `output: "standalone"` で軽量 Docker イメージを生成
