# Gengoka User Web — Architecture Specification

## 1. System Overview

```
┌─────────────────────────────────────────────────────────┐
│                     User Browser                         │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │              Next.js App (SPA-like)                 │  │
│  │                                                     │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │  │
│  │  │ Zustand   │  │ TanStack │  │  React Components │  │  │
│  │  │ Auth Store│  │ Query    │  │  (Pages/Layout)   │  │  │
│  │  └─────┬─────┘  └─────┬────┘  └────────┬─────────┘  │  │
│  │        │              │                 │             │  │
│  │        └──────────────┼─────────────────┘             │  │
│  │                       │                               │  │
│  │              ┌────────▼────────┐                      │  │
│  │              │   API Client    │                      │  │
│  │              │  (fetch wrapper)│                      │  │
│  │              └────────┬────────┘                      │  │
│  └───────────────────────┼────────────────────────────┘  │
│                          │                               │
│  ┌───────────────────────┼────────────────────────────┐  │
│  │        Next.js Middleware (Edge Runtime)             │  │
│  │   cookie check → redirect unauthenticated to /login │  │
│  └───────────────────────┼────────────────────────────┘  │
└──────────────────────────┼──────────────────────────────┘
                           │ HTTPS
                           ▼
              ┌────────────────────────┐
              │   Backend API Server   │
              │   (Rust / Actix-web)   │
              │   /api/v1/*            │
              └────────────────────────┘
```

Web フロントエンドは既存の Backend API をそのまま利用する。バックエンド変更は不要。モバイルアプリ（iOS/Android）と同一の API エンドポイント・認証フローを共有する。

---

## 2. Technology Stack

| レイヤー | 技術 | バージョン | 選定理由 |
|---|---|---|---|
| フレームワーク | Next.js (App Router) | 16.1.6 | SSR/SSG + ファイルベースルーティング + Edge Middleware |
| UI ランタイム | React | 19.2.3 | Server Components + Concurrent Features |
| 言語 | TypeScript | 5.x (strict) | 型安全性 |
| スタイリング | Tailwind CSS v4 | 4.x | CSS-first テーマ設定 + JIT |
| UI コンポーネント | shadcn/ui + Radix UI | — | アクセシブルなヘッドレス UI プリミティブ |
| サーバーステート | TanStack Query | 5.x | キャッシュ + 自動リフェッチ + mutation 管理 |
| クライアントステート | Zustand | 5.x | 軽量ストア + localStorage persist |
| フォーム | react-hook-form + Zod v4 | 7.x / 4.x | 非制御フォーム + スキーマバリデーション |
| アニメーション | framer-motion | 12.x | 宣言的アニメーション |
| アイコン | lucide-react | 0.575.x | Tree-shakable SVG アイコン |
| パッケージマネージャ | pnpm | 10.x | 高速 + ディスク効率 |

---

## 3. Directory Structure

```
sys/frontend/user/web/
├── public/                        # 静的アセット
├── src/
│   ├── app/                       # Next.js App Router
│   │   ├── (auth)/                # Route Group: 認証ページ
│   │   │   ├── layout.tsx         # 中央寄せグラデーション背景
│   │   │   ├── login/page.tsx
│   │   │   └── register/page.tsx
│   │   ├── (main)/                # Route Group: メイン画面
│   │   │   ├── layout.tsx         # Header + Sidebar + BottomNav
│   │   │   ├── home/page.tsx
│   │   │   ├── feed/page.tsx
│   │   │   ├── mypage/page.tsx
│   │   │   └── rankings/page.tsx
│   │   ├── globals.css            # Tailwind v4 テーマ定義
│   │   ├── layout.tsx             # ルートレイアウト
│   │   ├── page.tsx               # / → /login リダイレクト
│   │   └── providers.tsx          # TanStack Query Provider
│   ├── components/
│   │   ├── auth/                  # 認証関連コンポーネント
│   │   ├── common/                # 共通 UI コンポーネント
│   │   ├── layout/                # レイアウトコンポーネント
│   │   └── ui/                    # shadcn/ui 自動生成
│   ├── hooks/                     # カスタムフック
│   ├── lib/
│   │   ├── api/                   # API クライアント層
│   │   └── utils/                 # ユーティリティ関数
│   ├── stores/                    # Zustand ストア
│   ├── types/                     # TypeScript 型定義
│   └── middleware.ts              # Next.js Edge Middleware
├── .env.example
├── .env.local
├── next.config.ts
├── package.json
├── tsconfig.json
└── eslint.config.mjs
```

---

## 4. Authentication Architecture

### 4-1. トークン管理戦略

```
┌─────────────────────────────────────────────────┐
│                Token Storage                      │
│                                                   │
│  ┌──────────────────┐  ┌──────────────────────┐  │
│  │   Memory (JS)     │  │  localStorage         │  │
│  │                    │  │  key: 'gengoka-auth'  │  │
│  │  • accessToken    │  │                        │  │
│  │    (volatile)      │  │  • refreshToken       │  │
│  │                    │  │  • user (UserSummary)  │  │
│  └──────────────────┘  └──────────────────────┘  │
│                                                   │
│  ┌──────────────────────────────────────────────┐ │
│  │  Cookie                                       │ │
│  │  'gengoka-auth-present=1'                     │ │
│  │  (path=/; max-age=90d; SameSite=Lax)          │ │
│  │  → Middleware でのログイン状態判定用            │ │
│  └──────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

**設計判断:**
- `accessToken` をメモリのみに保持することで、XSS 攻撃時のトークン漏洩リスクを最小化
- `refreshToken` は localStorage に保持（ページリロード耐性）
- `gengoka-auth-present` cookie は軽量フラグ（トークン値は含まない）で、Edge Middleware がサーバーサイドで認証チェック可能にする
- モバイルアプリと同一の JWT + refresh token パターンを採用

### 4-2. 認証フロー

```
[初回アクセス]
  Browser → Middleware: cookie なし → redirect /login

[ログイン]
  LoginForm → useAuth.login() → POST /auth/login
    → Backend: AuthTokens { access_token, refresh_token, expires_in, user }
    → Zustand: setTokens() → cookie 設定 + state 更新
    → router.push('/home')

[ページリロード]
  Middleware: cookie あり → 通過
  AuthProvider mount: refreshToken あり + accessToken なし
    → POST /auth/refresh → setTokens()

[API コール]
  apiClient() → Authorization: Bearer <accessToken>
    → 401 応答 → refreshAccessToken() → リトライ
    → リフレッシュ失敗 → clearAuth() → /login へ

[定期リフレッシュ]
  AuthProvider: 25分間隔で POST /auth/refresh
    → 成功: トークン更新
    → 失敗: サイレント（次回 API コールで処理）

[ログアウト]
  useAuth.logout() → POST /auth/logout (onSettled)
    → clearAuth() → cookie 削除 → router.push('/login')
```

### 4-3. ソーシャルログイン（将来実装）

```
[OAuth フロー]（計画段階）
  SocialLoginButton → NextAuth.js OAuth redirect
    → Provider (Google/Apple/LINE) → callback
    → POST /auth/social { provider, id_token/access_token }
    → Backend: AuthTokens → setTokens()
```

現在は UI のみ実装済み。NextAuth.js v5 による OAuth リダイレクトフローは将来統合予定。

---

## 5. API Client Architecture

### 5-1. リクエストフロー

```
Component
  │
  ▼
TanStack Query (useQuery / useMutation)
  │
  ▼
apiClient<T>(endpoint, options) ─────── apiClientPaginated<T>()
  │                                            │
  │  1. URL 構築（base + endpoint + params）     │
  │  2. Authorization ヘッダー注入               │
  │  3. fetch()                                  │
  │  4. 401 → refreshAccessToken() → retry       │
  │  5. ApiResponse<T> をアンラップ               │
  │     → success: true → return data             │
  │     → success: false → throw ApiError         │
  │  6. HTTP 204 → return undefined               │
  ▼                                              ▼
return T                              return { data: T, pagination }
```

### 5-2. Backend API レスポンス形式

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 100,
    "total_pages": 5,
    "has_more": true
  }
}
```

API クライアントは `ApiResponse<T>` を自動的にアンラップし、コンポーネントには `T` のみを返す。

---

## 6. State Management

### 6-1. サーバーステート — TanStack Query

```
QueryClient 設定:
  staleTime: 60,000ms (1分)
  retry: 1

用途:
  - API データのキャッシュ
  - 自動リフェッチ（window focus, interval）
  - Optimistic updates
  - Pagination / Infinite scroll
```

### 6-2. クライアントステート — Zustand

```
auth-store:
  - accessToken (memory only)
  - refreshToken (persisted)
  - user (persisted)
  - setTokens / setUser / clearAuth

将来のストア:
  - ui-store: テーマ, サイドバー開閉, etc.
```

---

## 7. Routing & Layout

### 7-1. Route Groups

```
/                → redirect /login
(auth)/          → AuthLayout（中央寄せ、グラデーション背景）
  /login         → LoginForm
  /register      → RegisterForm
(main)/          → MainLayout（Header + Sidebar + BottomNav）
  /home          → デイリーチャレンジ（Phase 2）
  /feed          → タイムライン（Phase 3）
  /mypage        → マイページ（Phase 3）
  /rankings      → ランキング（Phase 4）
```

### 7-2. レスポンシブ戦略

```
                Mobile/Tablet              Desktop (lg:1024px+)
            ┌──────────────────┐      ┌──────────────────────────┐
            │     Header       │      │         Header            │
            ├──────────────────┤      ├─────────┬────────────────┤
            │                  │      │ Sidebar │                 │
            │   Main Content   │      │  w-56   │  Main Content   │
            │                  │      │         │                 │
            ├──────────────────┤      │         │                 │
            │    BottomNav     │      │         │                 │
            └──────────────────┘      └─────────┴────────────────┘
```

- **Mobile/Tablet**: BottomNav 表示、Sidebar 非表示
- **Desktop (lg:1024px+)**: Sidebar 表示、BottomNav 非表示
- ブレークポイント: `lg:1024px`

---

## 8. Design System

### 8-1. カラーパレット

| 名前 | Hex | 用途 |
|---|---|---|
| Primary Gradient Start | `#667eea` | ブランドカラー（開始） |
| Primary Gradient End | `#764ba2` | ブランドカラー（終了） |
| Text Primary | `#1a1a2e` | 見出し・本文 |
| Text Secondary | `#666666` | 補足テキスト |
| Text Tertiary | `#999999` | プレースホルダー・ヒント |
| Surface | `#F8F9FA` | ページ背景 |
| Border | `#e0e0e0` | ボーダー・区切り線 |
| Success | `#4CAF50` | 成功状態 |
| Warning | `#FFCC00` | 警告状態 |
| Error | `#E53935` | エラー状態 |

### 8-2. カテゴリカラー

| カテゴリ | Hex |
|---|---|
| 状況説明 (Situation) | `#FF9500` |
| 要約 (Summary) | `#007AFF` |
| 感情表現 (Emotion) | `#FF2D55` |
| 言い換え (Rephrase) | `#34C759` |
| 説明 (Explain) | `#AF52DE` |

モバイルアプリ（iOS `AppColors.swift` / Android `Color.kt`）と統一。

### 8-3. フォント

- **Noto Sans JP** — `next/font/google` 経由、`display: 'swap'`
- CSS 変数: `--font-noto-sans-jp`

### 8-4. 共通コンポーネント

| コンポーネント | 説明 |
|---|---|
| `GradientButton` | primary（グラデーション）/ secondary（アウトライン）。loading state 対応 |
| `UserAvatar` | 画像 or グラデーション背景イニシャルフォールバック |
| `LoadingSpinner` | スピンアイコン + メッセージ |
| `ErrorMessage` | エラーアイコン + メッセージ + リトライボタン |

---

## 9. Type System

Backend Rust モデル → TypeScript 型の 1:1 マッピング:

```
Backend (Rust)                  Web (TypeScript)
──────────────────             ──────────────────
models/mod.rs                  types/
  User, UserSummary              user.ts → UserSummary, UserProfile, UserStats, MyPageResponse
  UserProfile, MyPageResponse
  UserStats

  Challenge                      challenge.ts → Challenge, DailyChallengeResponse
  DailyChallengeResponse

  Answer, AiFeedback             answer.ts → Answer, AiFeedback, AnswerWithUser, AnswerWithDetails
  AnswerWithUser
  AnswerWithDetails

  Category, CategoryResponse     category.ts → Category, CategoryResponse

  Comment, CommentWithUser       comment.ts → Comment, CommentWithUser

  FeedQueryParams                feed.ts → FeedQueryParams, PaginationParams, AnswerQueryParams

  AuthTokens, LoginRequest       auth.ts → AuthTokens, LoginRequest, SignupRequest,
  SignupRequest, etc.                       RefreshRequest, LogoutRequest, SocialLoginRequest

utils/mod.rs                   lib/api/types.ts
  ApiResponse<T>                 ApiResponse<T>, PaginationInfo, ApiError
  PaginationInfo

routes/mod.rs                  lib/api/endpoints.ts
  /api/v1/*                      AUTH, CATEGORIES, CHALLENGES, ANSWERS,
                                 COMMENTS, USERS, FEED, RANKINGS
```

---

## 10. Build & Deployment

### 10-1. ビルド設定

```
next.config.ts:
  output: 'standalone'   → 自己完結型ビルド（Docker 対応）
```

### 10-2. コマンド

| コマンド | 説明 |
|---|---|
| `pnpm dev` | 開発サーバー (localhost:3000) |
| `pnpm build` | プロダクションビルド |
| `pnpm start` | プロダクションサーバー |
| `pnpm lint` | ESLint 実行 |

### 10-3. 環境変数

| 変数 | 説明 | デフォルト |
|---|---|---|
| `NEXT_PUBLIC_API_URL` | Backend API ベース URL | `http://localhost:8080/api/v1` |
| `NEXTAUTH_URL` | NextAuth ベース URL | `http://localhost:3000` |
| `NEXTAUTH_SECRET` | NextAuth シークレット | — |

---

## 11. Implementation Phases

| Phase | 内容 | 状態 |
|---|---|---|
| **Phase 1** | プロジェクト初期構築 + 認証 + レイアウト | **完了** |
| Phase 2 | ホーム画面（デイリーチャレンジ + 回答） | 未着手 |
| Phase 3 | タイムライン + マイページ + ユーザープロフィール | 未着手 |
| Phase 4 | ランキング + 設定 + その他 | 未着手 |
