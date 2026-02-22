# User Web Phase 2 - ホーム + チャレンジ機能

## Overview

Phase 1（プロジェクト基盤 + 認証 + レイアウト）完了後、ホーム画面・デイリーチャレンジ・チャレンジ回答・AI フィードバック結果表示・カテゴリ別一覧を実装した。`page.tsx`（Server Component / メタデータ）→ `XxxContent.tsx`（Client Component / データフェッチ + UI）のパターンを確立。

---

## Step 1: ホーム画面

### 1-1. HomeContent (`app/(main)/home/HomeContent.tsx`)

`'use client'` コンポーネント。3つのセクションを縦に配置:

1. **StatsBar** — ユーザー学習統計
2. **DailyChallengeList** — 今日のチャレンジ一覧
3. **CategoryGrid** — カテゴリ一覧

### 1-2. StatsBar (`components/home/StatsBar.tsx`)

`useUserStats()` フックで `/users/me/stats` を取得。4列グリッド:

| アイコン | ラベル | キー | 色 |
|---|---|---|---|
| Flame | 連続 | `current_streak` | #FF9500 |
| CheckCircle2 | 本日 | `completed_today` | #34C759 |
| Star | 平均 | `average_score` | #FFD700 |
| FileText | 総数 | `total_challenges` | #667eea |

- `average_score` は `toFixed(1)` で小数1桁表示
- ローディング時: 4つのスケルトンアニメーション

### 1-3. useUserStats フック (`hooks/useUserStats.ts`)

- `useQuery(['user-stats'], GET /users/me/stats)`
- `accessToken` が存在する場合のみ有効（`enabled: !!accessToken`）

---

## Step 2: デイリーチャレンジ

### 2-1. DailyChallengeList (`components/challenge/DailyChallengeList.tsx`)

`useDailyChallenges()` で `/challenges/daily` を取得。

- 水平スクロール（`overflow-x-auto snap-x`）
- 未完了チャレンジがある場合: 「残り N 問」バッジ表示
- 全問完了: PartyPopper アイコン + 「お疲れ様でした！」メッセージ
- ローディング時: 3つのスケルトンカード

### 2-2. ChallengeCard (`components/challenge/ChallengeCard.tsx`)

`DailyChallengeResponse` を受け取り、カード型で表示:

- カテゴリ名バッジ（カテゴリカラー背景）
- チャレンジタイトル（2行 line-clamp）
- 文字数制限表示
- 完了済み: 緑チェックマークオーバーレイ
- ホバー: `scale(1.02)` アニメーション
- `Link` で `/challenges/{id}` へ遷移

### 2-3. useChallenges フック (`hooks/useChallenges.ts`)

| フック | API | 用途 |
|---|---|---|
| `useDailyChallenges()` | `GET /challenges/daily` | 今日のチャレンジ一覧 |
| `useChallengeDetail(id)` | `GET /challenges/{id}` | チャレンジ詳細 |
| `useSubmitAnswer(challengeId)` | `POST /challenges/{id}/answers` | 回答送信 |
| `useAnswerDetail(answerId)` | `GET /answers/{id}` | 回答詳細 |

`useSubmitAnswer` の成功時:
- `['daily-challenges']` と `['user-stats']` のキャッシュ無効化
- `toast.success('回答を送信しました')`
- `/challenges/{id}/result?answerId={answer.id}` へ遷移

エラー時:
- 409 → `toast.error('この問題は回答済みです')`
- その他 → `toast.error('送信に失敗しました')`

---

## Step 3: チャレンジ画面 + 結果画面

### 3-1. ChallengeContent (`app/(main)/challenges/[id]/ChallengeContent.tsx`)

- `useParams<{ id: string }>()` でルートパラメータ取得
- `useChallengeDetail(id)` でチャレンジ詳細を取得
- `useDailyChallenges()` から既存回答を検索
- カテゴリバッジ（`CATEGORY_COLORS` でカテゴリ色付け）
- 戻るリンク（ArrowLeft アイコン → `/home`）
- `AnswerForm` にチャレンジと既存回答を渡す

### 3-2. AnswerForm (`components/challenge/AnswerForm.tsx`)

2つの状態:

**未回答:**
- テキストエリア（6行、プレースホルダー付き）
- `CharacterCounter` でリアルタイム文字数表示
- 送信ボタン（`GradientButton` + ローディング状態）
- バリデーション: 空でない + 文字数制限内

**回答済み:**
- 既存回答を読み取り専用で表示
- 「結果を見る」リンク → `/challenges/{id}/result?answerId={answer.id}`

### 3-3. CharacterCounter (`components/challenge/CharacterCounter.tsx`)

`{current} / {max}` 形式で文字数を表示:
- 空: グレー（#999999）
- 範囲内: 緑（#34C759）
- 超過: 赤（#E53935）

### 3-4. ResultContent (`app/(main)/challenges/[id]/result/ResultContent.tsx`)

- URL パラメータ `answerId` で回答を取得
- `useAnswerDetail(answerId)` でデータフェッチ
- `ResultDisplay` にデータを渡す

### 3-5. ResultDisplay (`components/challenge/ResultDisplay.tsx`)

スコアと AI フィードバックを表示:

**スコア表示:**
- `ScoreCircle` — SVG 円形プログレス + framer-motion アニメーション（1.5秒）
- スコア範囲別メッセージ:
  - 90+ → 「素晴らしい！」
  - 80+ → 「とても良くできました！」
  - 70+ → 「良い回答です！」
  - 60+ → 「もう少し工夫してみましょう」
  - 60 未満 → 「がんばりましょう！」

**AI フィードバック（3セクション）:**
- 良い点（緑 `#34C759` 左ボーダー）
- 改善点（オレンジ `#FF9500` 左ボーダー）
- 例文（青 `#007AFF` 左ボーダー）

**フッター:**
- 元の回答テキスト表示
- 「ホームに戻る」ボタン

### 3-6. ScoreCircle (`components/challenge/ScoreCircle.tsx`)

SVG 円形スコア表示:
- 140×140 viewBox、半径 54px
- グラデーションストローク（`#667eea` → `#764ba2`）
- `framer-motion` の `useMotionValue` + `useTransform` でアニメーション
- `offset = circumference - (score / 100) * circumference` で進捗計算

---

## Step 4: カテゴリ別チャレンジ一覧

### 4-1. CategoryGrid (`components/category/CategoryGrid.tsx`)

`useCategories()` で `/categories` を取得。

- 2列グリッド（`grid-cols-2 gap-3`）
- カード: アイコン（絵文字）+ カテゴリ名 + 説明（2行 clamp）+ チャレンジ数
- カテゴリカラーで背景 tint
- ホバー: `scale(1.02)` アニメーション
- `Link` で `/categories/{id}` へ遷移
- ローディング時: 4つのスケルトン

### 4-2. CategoryContent (`app/(main)/categories/[id]/CategoryContent.tsx`)

- `useCategoryDetail(id)` でカテゴリ情報取得
- `useCategoryChallenges(id, page)` でページネーション付きチャレンジ一覧
- カテゴリヘッダー: アイコン + 名前 + 説明 + チャレンジ数
- チャレンジリスト: タイトル + 説明 + 文字数制限 + 回答数
- ページネーション: 「前へ / 次へ」ボタン + 現在ページ / 総ページ数

### 4-3. useCategories フック (`hooks/useCategories.ts`)

| フック | API | 用途 |
|---|---|---|
| `useCategories()` | `GET /categories` | 全カテゴリ一覧 |
| `useCategoryDetail(id)` | `GET /categories/{id}` | カテゴリ詳細 |
| `useCategoryChallenges(id, page)` | `GET /categories/{id}/challenges` | カテゴリ別チャレンジ（ページネーション） |

全て `noAuth: true`（公開 API）。`useCategoryChallenges` は `apiClientPaginated` を使用（`page_size: 10`）。

---

## 新規ファイル一覧

### Hooks (3)
| ファイル | 内容 |
|---|---|
| `hooks/useChallenges.ts` | デイリーチャレンジ・詳細・回答送信・回答詳細 |
| `hooks/useCategories.ts` | カテゴリ一覧・詳細・チャレンジ一覧 |
| `hooks/useUserStats.ts` | ユーザー学習統計 |

### Components (8)
| ファイル | 内容 |
|---|---|
| `components/home/StatsBar.tsx` | 4列学習統計グリッド |
| `components/challenge/DailyChallengeList.tsx` | 水平スクロールデイリーチャレンジ |
| `components/challenge/ChallengeCard.tsx` | チャレンジカード（カテゴリ色、完了マーク） |
| `components/challenge/AnswerForm.tsx` | 回答入力フォーム（文字数制限付き） |
| `components/challenge/CharacterCounter.tsx` | 文字数カウンター（色変化） |
| `components/challenge/ScoreCircle.tsx` | SVG 円形スコアアニメーション |
| `components/challenge/ResultDisplay.tsx` | AI フィードバック結果画面 |
| `components/category/CategoryGrid.tsx` | 2列カテゴリグリッド |

### Pages (7)
| ファイル | 内容 |
|---|---|
| `app/(main)/home/page.tsx` | ホームページ（Server Component） |
| `app/(main)/home/HomeContent.tsx` | ホーム Client Component |
| `app/(main)/challenges/[id]/page.tsx` | チャレンジページ |
| `app/(main)/challenges/[id]/ChallengeContent.tsx` | チャレンジ Client Component |
| `app/(main)/challenges/[id]/result/page.tsx` | 結果ページ |
| `app/(main)/challenges/[id]/result/ResultContent.tsx` | 結果 Client Component |
| `app/(main)/categories/[id]/page.tsx` + `CategoryContent.tsx` | カテゴリ詳細ページ |

---

## 設計パターン

- **Page / Content 分離**: `page.tsx`（メタデータ + Server Component）→ `XxxContent.tsx`（`'use client'` + データフェッチ + UI）
- **React Query**: サーバーステート管理（キャッシュ、無効化、楽観的更新）
- **apiClient / apiClientPaginated**: 統一 API クライアント（自動トークン注入 + 401 リトライ）
- **共通コンポーネント**: `LoadingSpinner`, `ErrorMessage`, `GradientButton`, `UserAvatar`
- **Toast 通知**: Sonner で成功/エラーフィードバック
- **カテゴリカラー**: `CATEGORY_COLORS` 定数でカテゴリごとに色分け

## Build Status

- `pnpm build` 成功
- 全ルート: `/home`, `/feed`(placeholder), `/mypage`(placeholder), `/rankings`(placeholder), `/challenges/[id]`, `/challenges/[id]/result`, `/categories/[id]`
