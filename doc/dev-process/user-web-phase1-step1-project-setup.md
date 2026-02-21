# User Web Phase 1 - Project Setup, Auth, Layout

## Overview

モバイルアプリ（iOS/Android）で提供中の全機能を Web でも利用可能にするため、Next.js フロントエンドを新規構築した。既存 Backend API (`/api/v1/*`) をそのまま利用し、バックエンド変更は不要。

---

## Step 1: Project Initialization

### 1-1. Next.js プロジェクト作成

```bash
pnpm create next-app@latest . --typescript --tailwind --eslint --app --src-dir --import-alias "@/*" --use-pnpm
```

- Next.js 16.1.6 + React 19.2.3
- App Router（`src/app/`）
- Tailwind CSS v4（CSS-first `@theme` 設定）
- TypeScript 5.x strict mode

### 1-2. 依存関係

| パッケージ | バージョン | 用途 |
|---|---|---|
| `@tanstack/react-query` | ^5.90.21 | サーバーステート管理 / データフェッチ |
| `zustand` | ^5.0.11 | クライアントステート（認証ストア） |
| `react-hook-form` | ^7.71.2 | フォーム管理 |
| `@hookform/resolvers` | ^5.2.2 | Zod バリデーション統合 |
| `zod` | ^4.3.6 | スキーマバリデーション |
| `framer-motion` | ^12.34.3 | アニメーション |
| `radix-ui` | ^1.4.3 | ヘッドレス UI プリミティブ |
| `class-variance-authority` | ^0.7.1 | コンポーネントバリアントシステム |
| `clsx` + `tailwind-merge` | ^2.1.1 / ^3.5.0 | クラス名ユーティリティ |
| `lucide-react` | ^0.575.0 | アイコンセット |
| `sonner` | ^2.0.7 | トースト通知 |
| `next-themes` | ^0.4.6 | テーマサポート |

**Dev 依存:**
- `tailwindcss` ^4 / `@tailwindcss/postcss` ^4
- `shadcn` ^3.8.5（UI コンポーネント生成）
- `prettier` ^3.8.1 / `eslint-config-prettier` ^10.1.8
- `tw-animate-css` ^1.4.0

### 1-3. shadcn/ui コンポーネント

以下の UI コンポーネントを追加:
- `button` — CVA バリアント（default/destructive/outline/secondary/ghost/link）
- `input` — テキスト入力（focus/invalid リングスタイル付き）
- `label` — Radix Label.Root
- `card` — Card/CardHeader/CardTitle/CardDescription/CardContent/CardFooter
- `separator` — 水平/垂直セパレータ
- `dropdown-menu` — 完全な Radix DropdownMenu セット
- `avatar` — Avatar/AvatarImage/AvatarFallback（sm/default/lg サイズ）
- `sheet` — Radix Dialog ベースのスライドパネル（top/right/bottom/left）
- `sonner` — テーマ対応トースト通知

### 1-4. 設定ファイル

**`next.config.ts`:**
- `output: 'standalone'` — Docker 対応ビルド
- `images.remotePatterns` — `*.googleusercontent.com`, `*.apple.com` を許可

**`eslint.config.mjs`:**
- `eslint-config-next/core-web-vitals` + `eslint-config-next/typescript` + `prettier` 統合

**`.prettierrc`:**
- `semi: true`, `singleQuote: true`, `tabWidth: 2`, `trailingComma: 'all'`, `printWidth: 100`

**`.env.example` / `.env.local`:**
- `NEXT_PUBLIC_API_URL` — Backend API ベース URL
- `NEXTAUTH_URL` / `NEXTAUTH_SECRET` — NextAuth 設定（OAuth 用、将来実装）

### 1-5. TypeScript 型定義

Backend `src/models/mod.rs` の全型を TypeScript に 1:1 マッピング:

| ファイル | 型 |
|---|---|
| `src/types/auth.ts` | `AuthTokens`, `LoginRequest`, `SignupRequest`, `RefreshRequest`, `LogoutRequest`, `SocialLoginRequest` |
| `src/types/user.ts` | `UserSummary`, `UserProfile`, `UserStats`, `MyPageResponse`, `UpdateUserRequest` |
| `src/types/answer.ts` | `Answer`, `AiFeedback`, `AnswerWithUser`, `AnswerWithDetails`, `CreateAnswerRequest` |
| `src/types/challenge.ts` | `Challenge`, `DailyChallengeResponse` |
| `src/types/category.ts` | `Category`, `CategoryResponse` |
| `src/types/comment.ts` | `Comment`, `CommentWithUser`, `CreateCommentRequest` |
| `src/types/feed.ts` | `FeedQueryParams`, `PaginationParams`, `AnswerQueryParams` |

### 1-6. API クライアント

**`src/lib/api/types.ts`:**
```typescript
ApiResponse<T>  { success, data: T|null, error?, pagination?: PaginationInfo }
PaginationInfo  { page, page_size, total, total_pages, has_more }
class ApiError extends Error { statusCode: number }
```

**`src/lib/api/endpoints.ts`:**
Backend `routes/mod.rs` と完全一致するエンドポイント定義:
- `AUTH` — REGISTER, LOGIN, REFRESH, LOGOUT, SOCIAL
- `CATEGORIES` — LIST, GET(id), CHALLENGES(id)
- `CHALLENGES` — LIST, DAILY, GET(id), ANSWERS(id)
- `ANSWERS` — GET(id), UPDATE(id), DELETE(id), LIKE(id), UNLIKE(id), COMMENTS(id)
- `COMMENTS` — DELETE(id)
- `USERS` — ME, MY_STATS, MY_SOCIAL_ACCOUNTS, UNLINK_SOCIAL(provider), GET(id), UPDATE(id), ANSWERS(id), FOLLOW(id), UNFOLLOW(id), FOLLOWERS(id), FOLLOWING(id)
- `FEED` — LIST, TRENDING
- `RANKINGS` — DAILY, WEEKLY, ALL_TIME

**`src/lib/api/client.ts`:**
- `apiClient<T>(endpoint, options)` — fetch ラッパー。`ApiResponse<T>` を自動アンラップして `T` を返す
  - `Authorization: Bearer <token>` を Zustand ストアから自動注入
  - HTTP 401 時: `refreshAccessToken()` を呼び出し、新トークンでリトライ
  - HTTP 204 No Content を処理
  - `params` オプションでクエリパラメータを自動付与
  - `noAuth: true` で認証ヘッダーをスキップ
- `apiClientPaginated<T>(endpoint, options)` — `{ data: T, pagination: PaginationInfo }` を返す

### 1-7. TanStack Query Provider

**`src/app/providers.tsx`:**
- `QueryClient` — `staleTime: 60000`（1分）、`retry: 1`
- `QueryClientProvider` でアプリ全体をラップ

---

## Step 2: Authentication

### 2-1. Zustand 認証ストア (`src/stores/auth-store.ts`)

```
State:
  accessToken: string | null    ← メモリのみ（persist しない）
  refreshToken: string | null   ← localStorage に persist
  user: UserSummary | null      ← localStorage に persist

Actions:
  setTokens(accessToken, refreshToken, user)
    → cookie 'gengoka-auth-present=1' を設定（90日、SameSite=Lax）
  setUser(user)
  clearAuth()
    → cookie 削除 + state クリア

localStorage key: 'gengoka-auth'
persist対象: refreshToken + user のみ（partialize で accessToken を除外）
```

### 2-2. バリデーション (`src/lib/utils/validation.ts`)

Zod v4 スキーマ（全エラーメッセージ日本語）:

| スキーマ | フィールド | バリデーション |
|---|---|---|
| `loginSchema` | `email` | 有効なメールアドレス |
| | `password` | 必須（min 1） |
| `registerSchema` | `name` | 1〜100文字 |
| | `email` | 有効なメールアドレス |
| | `password` | 8文字以上 |
| | `confirmPassword` | password と一致（`.refine()` で検証） |

### 2-3. useAuth フック (`src/hooks/useAuth.ts`)

TanStack Query `useMutation` ベース:

| メソッド | API | 成功時 |
|---|---|---|
| `login(LoginRequest)` | `POST /auth/login` | `setTokens` → `/home` 遷移 |
| `register(SignupRequest)` | `POST /auth/register` | `setTokens` → `/home` 遷移 |
| `socialLogin(SocialLoginRequest)` | `POST /auth/social` | `setTokens` → `/home` 遷移 |
| `logout()` | `POST /auth/logout` | `clearAuth` → `/login` 遷移（`onSettled`） |

返却値: 各メソッド + `isLoggingIn`, `isRegistering`, `isSocialLogging`, `isLoggingOut`, `loginError`, `registerError`

### 2-4. ログイン画面

**`src/app/(auth)/layout.tsx`:**
- 中央寄せ `flex` + グラデーション背景（`from-[#667eea]/10 via-white to-[#764ba2]/10`）

**`src/app/(auth)/login/page.tsx`:**
- `<LoginForm />` を表示。メタデータ: "ログイン - Gengoka"

**`src/components/auth/LoginForm.tsx`:**
- `react-hook-form` + `zodResolver(loginSchema)`
- ソーシャルログインボタン → セパレータ「または」→ メール/パスワードフォーム
- エラー表示: 401 → 「メールアドレスまたはパスワードが正しくありません」
- フッター: 「アカウントをお持ちでないですか？ → 新規登録」リンク

### 2-5. 新規登録画面

**`src/app/(auth)/register/page.tsx`:**
- `<RegisterForm />` を表示。メタデータ: "新規登録 - Gengoka"

**`src/components/auth/RegisterForm.tsx`:**
- フィールド: 名前 / メールアドレス / パスワード / パスワード（確認）
- エラー表示: 409 → 「このメールアドレスは既に登録されています」
- フッター: 「既にアカウントをお持ちですか？ → ログイン」リンク

### 2-6. ソーシャルログインボタン (`src/components/auth/SocialLoginButtons.tsx`)

3つのブランドボタン（インライン SVG アイコン付き）:
- **Google** — 白背景、Google 4色ロゴ
- **Apple** — 黒背景、Apple ロゴ（白）
- **LINE** — `#06C755` 背景、LINE ロゴ（白）

ハンドラーは `console.log` スタブ（NextAuth OAuth 統合は将来実装）。

### 2-7. 認証ミドルウェア (`src/middleware.ts`)

```
公開パス: /login, /register, /api/auth
除外: _next/static, _next/image, favicon.ico, 画像ファイル

ロジック:
  1. 公開パスに該当 → 通過
  2. cookie 'gengoka-auth-present' を確認
     - なし → /login へリダイレクト
     - あり → 通過
```

### 2-8. AuthProvider (`src/components/auth/AuthProvider.tsx`)

- マウント時: `refreshToken` あり + `accessToken` なし → `POST /auth/refresh` でトークン取得
- 25分間隔: `setInterval` で定期リフレッシュ（`refreshToken` と `accessToken` の両方が存在する場合のみ）
- アンマウント時: interval クリーンアップ
- UI なし（`{children}` をそのままレンダリング）

---

## Step 3: Layout & Design System

### 3-1. Tailwind CSS テーマ (`src/app/globals.css`)

Tailwind v4 の `@theme inline` でカスタムカラー定義:

| 変数 | 値 | 用途 |
|---|---|---|
| `--color-gengoka-primary-start` | `#667eea` | グラデーション開始 |
| `--color-gengoka-primary-end` | `#764ba2` | グラデーション終了 |
| `--color-gengoka-text` | `#1a1a2e` | プライマリテキスト |
| `--color-gengoka-text-secondary` | `#666666` | セカンダリテキスト |
| `--color-gengoka-text-tertiary` | `#999999` | ターシャリテキスト |
| `--color-gengoka-surface` | `#F8F9FA` | 背景サーフェス |
| `--color-gengoka-border` | `#e0e0e0` | ボーダー |
| `--color-gengoka-success` | `#4CAF50` | 成功 |
| `--color-gengoka-warning` | `#FFCC00` | 警告 |
| `--color-gengoka-error` | `#E53935` | エラー |
| `--color-category-situation` | `#FF9500` | 状況説明カテゴリ |
| `--color-category-summary` | `#007AFF` | 要約カテゴリ |
| `--color-category-emotion` | `#FF2D55` | 感情表現カテゴリ |
| `--color-category-rephrase` | `#34C759` | 言い換えカテゴリ |
| `--color-category-explain` | `#AF52DE` | 説明カテゴリ |

フォント: `--font-sans` → `--font-noto-sans-jp`（Noto Sans JP）

### 3-2. ルートレイアウト (`src/app/layout.tsx`)

- `Noto_Sans_JP` フォント（`next/font/google`、CSS 変数 `--font-noto-sans-jp`）
- `lang="ja"`, `font-sans antialiased`
- ラッピング順: `<Providers>` → `<AuthProvider>` → children + `<Toaster />`
- メタデータ: title "Gengoka - 言語化トレーニング", description "言語化力を鍛えるトレーニングプラットフォーム"

### 3-3. メインレイアウト (`src/app/(main)/layout.tsx`)

- 背景: `bg-[#F8F9FA]`
- 構成: `<Header />` → `<Sidebar />` + `<main>` 横並び → `<BottomNav />`
- `max-w-7xl` で中央寄せ
- `main` に `pb-20 lg:pb-6`（モバイルでの BottomNav 分の余白）

### 3-4. レイアウトコンポーネント

**Header (`src/components/layout/Header.tsx`):**
- `sticky top-0 z-40`、`h-14`、白背景 + `backdrop-blur-sm`
- 左: "Gengoka" グラデーションテキストロゴ → `/home`
- 右: `UserAvatar` + `DropdownMenu`（マイページ / 設定 / ログアウト）

**Sidebar (`src/components/layout/Sidebar.tsx`):**
- `lg:block hidden`（1024px 以上で表示）
- `sticky top-14`、`w-56`、右ボーダー
- 4項目ナビ: ホーム / タイムライン / マイページ / ランキング
- アクティブ状態: グラデーション背景 + `text-[#667eea]`

**BottomNav (`src/components/layout/BottomNav.tsx`):**
- `lg:hidden`（1024px 未満で表示）
- `fixed bottom-0`、`h-16`、上ボーダー
- Sidebar と同じ4項目（アイコン + 10px ラベル）

### 3-5. 共通コンポーネント

**GradientButton (`src/components/common/GradientButton.tsx`):**
- `forwardRef` ボタン、2バリアント:
  - `primary`: `bg-gradient-to-r from-[#667eea] to-[#764ba2]`、白テキスト、シャドウ
  - `secondary`: 白背景、`#667eea` ボーダー/テキスト
- `loading` prop: `Loader2` スピンアニメーション

**UserAvatar (`src/components/common/UserAvatar.tsx`):**
- shadcn `Avatar` ラッパー
- 画像あり → `AvatarImage` 表示
- 画像なし → グラデーション背景 + イニシャル2文字フォールバック

**LoadingSpinner (`src/components/common/LoadingSpinner.tsx`):**
- `Loader2` スピンアイコン + メッセージテキスト（デフォルト: "読み込み中..."）

**ErrorMessage (`src/components/common/ErrorMessage.tsx`):**
- `AlertCircle` アイコン + メッセージ + オプション「再試行」ボタン

### 3-6. ユーティリティ

**`src/lib/utils/constants.ts`:**
- `CATEGORY_COLORS` — カテゴリ名 → 色コードマッピング
- `NAV_ITEMS` — ナビゲーション項目定義（href, label, icon）

**`src/lib/utils/date.ts`:**
- `formatRelativeTime(dateString)` — 相対時間表示
  - `< 60秒` → "たった今"
  - `< 60分` → "N分前"
  - `< 24時間` → "N時間前"
  - `< 7日` → "N日前"
  - それ以外 → `ja-JP` ロケール日付

### 3-7. プレースホルダーページ

| パス | タイトル | 状態 |
|---|---|---|
| `/home` | ホーム - Gengoka | Phase 2 で実装予定 |
| `/feed` | タイムライン - Gengoka | Phase 3 で実装予定 |
| `/mypage` | マイページ - Gengoka | Phase 3 で実装予定 |
| `/rankings` | ランキング - Gengoka | Phase 4 で実装予定 |

ルート `/` は `/login` へ即座にリダイレクト。

---

## File Structure

```
sys/frontend/user/web/src/
├── app/
│   ├── (auth)/                    # 認証レイアウト（中央寄せグラデーション）
│   │   ├── layout.tsx
│   │   ├── login/page.tsx
│   │   └── register/page.tsx
│   ├── (main)/                    # メインレイアウト（Header+Sidebar+BottomNav）
│   │   ├── layout.tsx
│   │   ├── home/page.tsx
│   │   ├── feed/page.tsx
│   │   ├── mypage/page.tsx
│   │   └── rankings/page.tsx
│   ├── globals.css                # Tailwind v4 テーマ + カスタムカラー
│   ├── layout.tsx                 # ルートレイアウト（Noto Sans JP + Providers）
│   ├── page.tsx                   # / → /login リダイレクト
│   └── providers.tsx              # TanStack Query Provider
├── components/
│   ├── auth/
│   │   ├── AuthProvider.tsx       # トークンリフレッシュ管理
│   │   ├── LoginForm.tsx          # ログインフォーム
│   │   ├── RegisterForm.tsx       # 新規登録フォーム
│   │   └── SocialLoginButtons.tsx # Google/Apple/LINE ボタン
│   ├── common/
│   │   ├── ErrorMessage.tsx       # エラー表示 + リトライ
│   │   ├── GradientButton.tsx     # primary/secondary グラデーションボタン
│   │   ├── LoadingSpinner.tsx     # ローディングスピナー
│   │   └── UserAvatar.tsx         # アバター + イニシャルフォールバック
│   ├── layout/
│   │   ├── BottomNav.tsx          # モバイル用ボトムナビ
│   │   ├── Header.tsx             # ヘッダー + ユーザーメニュー
│   │   └── Sidebar.tsx            # PC用サイドバー
│   └── ui/                        # shadcn/ui 自動生成
│       ├── avatar.tsx
│       ├── button.tsx
│       ├── card.tsx
│       ├── dropdown-menu.tsx
│       ├── input.tsx
│       ├── label.tsx
│       ├── separator.tsx
│       ├── sheet.tsx
│       └── sonner.tsx
├── hooks/
│   └── useAuth.ts                 # 認証 mutation フック
├── lib/
│   ├── api/
│   │   ├── client.ts              # API クライアント（auto-unwrap + 401 retry）
│   │   ├── endpoints.ts           # 全エンドポイントパス定義
│   │   └── types.ts               # ApiResponse<T>, PaginationInfo, ApiError
│   ├── utils/
│   │   ├── constants.ts           # カテゴリカラー、ナビ項目
│   │   ├── date.ts                # 相対時間フォーマット
│   │   └── validation.ts          # Zod スキーマ（日本語エラー）
│   └── utils.ts                   # cn() ユーティリティ
├── stores/
│   └── auth-store.ts              # Zustand 認証ストア
├── types/
│   ├── answer.ts
│   ├── auth.ts
│   ├── category.ts
│   ├── challenge.ts
│   ├── comment.ts
│   ├── feed.ts
│   └── user.ts
└── middleware.ts                   # 認証ミドルウェア
```

---

## Build Status

- `pnpm build` 成功（全10ルートコンパイル済み）
- 全ルート: `/`, `/_not-found`, `/feed`, `/home`, `/login`, `/mypage`, `/rankings`, `/register`
