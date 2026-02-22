# User Web Phase 5 - テスト + デプロイ

## Overview

Phase 4（ランキング）完了後、ユニットテスト基盤、E2E テスト基盤、Docker コンテナ化、GitHub Actions CI/CD パイプラインを構築した。

---

## Step 1: ユニットテスト

### 1-1. Vitest セットアップ

**依存パッケージ追加:**
- `vitest` — テストランナー
- `@vitejs/plugin-react` — React JSX サポート
- `jsdom` — DOM 環境
- `@testing-library/react` — React コンポーネントテスト
- `@testing-library/jest-dom` — DOM マッチャー拡張
- `@testing-library/user-event` — ユーザーイベントシミュレーション

**設定ファイル (`vitest.config.ts`):**
- `environment: 'jsdom'` — DOM テスト環境
- `setupFiles: ['./tests/setup.ts']` — jest-dom マッチャー登録
- `globals: true` — describe/it/expect をグローバルに
- `@/` パスエイリアス解決

### 1-2. テストファイル一覧

| ファイル | テスト数 | テスト対象 |
|---|---|---|
| `tests/unit/date.test.ts` | 8 | `formatRelativeTime` — たった今、分前、時間前、日前、日付フォーマット、境界値 |
| `tests/unit/validation.test.ts` | 13 | `loginSchema`, `registerSchema` — メール、パスワード、名前、一致確認、境界値 |
| `tests/unit/api-types.test.ts` | 3 | `ApiError` クラス — インスタンス生成、Error 継承、ステータスコード保持 |
| `tests/unit/constants.test.ts` | 7 | `CATEGORY_COLORS`（5色マッピング + 未知カテゴリ）、`NAV_ITEMS`（4項目確認） |
| `tests/unit/endpoints.test.ts` | 12 | 全エンドポイント定数 — 静的パス + 動的パス生成（AUTH, CATEGORIES, CHALLENGES, ANSWERS, COMMENTS, USERS, FEED, RANKINGS） |
| `tests/unit/api-client.test.ts` | 12 | `apiClient` — GET, 認証ヘッダー, noAuth, クエリパラメータ, POST ボディ, HTTP エラー, ApiError ステータスコード, success:false, 204 No Content。`apiClientPaginated` — ページネーション, エラー |
| **合計** | **55** | |

### 1-3. 主要テストパターン

- **Zustand モック**: `vi.mock('@/stores/auth-store')` でストアを固定値に
- **fetch モック**: `vi.stubGlobal('fetch', vi.fn())` + `mockResolvedValueOnce`
- **タイマー制御**: `vi.useFakeTimers()` + `vi.setSystemTime()` で日付テスト
- **Zod バリデーション**: `schema.safeParse()` で成功/失敗を検証

---

## Step 2: E2E テスト

### 2-1. Playwright セットアップ

**依存パッケージ追加:**
- `@playwright/test` — E2E テストフレームワーク
- `playwright` — ブラウザ自動化

**設定ファイル (`playwright.config.ts`):**
- `baseURL: 'http://localhost:3000'`
- `projects`: Desktop Chrome + iPhone 14（レスポンシブ）
- `webServer`: `pnpm dev` を自動起動
- CI 環境: リトライ2回、ワーカー1

### 2-2. E2E テストファイル

| ファイル | テスト内容 |
|---|---|
| `tests/e2e/auth.spec.ts` | 未認証リダイレクト、ログイン画面表示、登録画面表示、バリデーションエラー、ページ間ナビゲーション |
| `tests/e2e/navigation.spec.ts` | ページタイトル確認、保護ルートのリダイレクト確認（/home, /feed, /mypage, /rankings） |

---

## Step 3: Docker デプロイ

### 3-1. Dockerfile（マルチステージビルド）

```
Stage 1: deps    — node:22-alpine, pnpm install --frozen-lockfile
Stage 2: builder — COPY + pnpm build
Stage 3: runner  — standalone output + non-root user (nextjs:nodejs)
```

- `output: "standalone"` で軽量イメージ生成
- 非 root ユーザー（UID 1001）で実行
- ポート 3000、ホスト `0.0.0.0`

### 3-2. .dockerignore

除外: `node_modules`, `.next`, `.git`, `tests`, テスト設定ファイル, `.env.local`

### 3-3. docker-compose.prod.yml 更新

既存の `frontend`（nginx:alpine 静的配信）を `web`（Next.js SSR）に置換:

```yaml
web:
  build:
    context: ../frontend/user/web
    dockerfile: Dockerfile
  container_name: gengoka-web
  env_file: .env.web
  environment:
    - NEXT_PUBLIC_API_URL=https://backend.gengoka.app/api/v1
  networks: [gengoka-network, traefik-network]
  depends_on: [backend]
  labels:
    - traefik.http.routers.gengoka-web.rule=Host(`gengoka.app`)
    - traefik.http.services.gengoka-web.loadbalancer.server.port=3000
  deploy.resources.limits.memory: 512M
```

### 3-4. GitHub Actions ワークフロー

**CI (`web-ci.yml`):**
- トリガー: `sys/frontend/user/web/**` のプッシュ / PR
- ステップ: pnpm install → lint → type-check → unit test → build

**CD (`web-deploy.yml`):**
- トリガー: main ブランチへのプッシュ（`sys/frontend/user/web/**`）
- ステップ: SSH → git pull → docker compose build web → up -d web
- シークレット: `VPS_HOST`, `VPS_USER`, `VPS_SSH_KEY`, `PROJECT_PATH`

---

## package.json スクリプト追加

```json
"test": "vitest run",
"test:watch": "vitest",
"test:e2e": "playwright test",
"test:e2e:ui": "playwright test --ui"
```

---

## 新規ファイル一覧

### テスト基盤 (3)
| ファイル | 内容 |
|---|---|
| `vitest.config.ts` | Vitest 設定（jsdom, React, パスエイリアス） |
| `playwright.config.ts` | Playwright 設定（Chrome + iPhone、dev server 自動起動） |
| `tests/setup.ts` | jest-dom マッチャー登録 |

### ユニットテスト (6)
| ファイル | テスト数 |
|---|---|
| `tests/unit/date.test.ts` | 8 |
| `tests/unit/validation.test.ts` | 13 |
| `tests/unit/api-types.test.ts` | 3 |
| `tests/unit/constants.test.ts` | 7 |
| `tests/unit/endpoints.test.ts` | 12 |
| `tests/unit/api-client.test.ts` | 12 |

### E2E テスト (2)
| ファイル | 内容 |
|---|---|
| `tests/e2e/auth.spec.ts` | 認証フロー |
| `tests/e2e/navigation.spec.ts` | ナビゲーション + 保護ルート |

### デプロイ (2)
| ファイル | 内容 |
|---|---|
| `Dockerfile` | マルチステージビルド（deps → builder → runner） |
| `.dockerignore` | Docker ビルド除外ファイル |

### CI/CD (2)
| ファイル | 内容 |
|---|---|
| `.github/workflows/web-ci.yml` | lint + type-check + test + build |
| `.github/workflows/web-deploy.yml` | VPS SSH デプロイ |

### 変更 (2)
| ファイル | 変更内容 |
|---|---|
| `package.json` | test スクリプト追加 + devDependencies 追加 |
| `docker-compose.prod.yml` | frontend (nginx) → web (Next.js) 置換 |

---

## テスト結果

```
Test Files   6 passed (6)
     Tests   55 passed (55)
  Duration   1.6s
```

## Build Status

- `pnpm build` 成功（全14ルート）
- `pnpm test` 成功（55テスト全パス）
- 全 Phase 1〜5 実装完了
