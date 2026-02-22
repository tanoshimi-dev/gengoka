# User Web Phase 4 - ランキング

## Overview

Phase 3（フィード + ソーシャル）完了後、ランキング画面を実装した。デイリー / ウィークリー / 累計の3タブ切り替え、無限スクロール、トップ3メダル表示、自分のランキングハイライトを含む。Phase 3 で確立した `useInfiniteQuery` + `IntersectionObserver` パターン、タブ切り替えパターンを踏襲。

---

## Step 1: ランキング画面

### 1-1. RankingsContent (`app/(main)/rankings/RankingsContent.tsx`)

ランキングページの Client Component 本体。

- `RankingTabs` で期間フィルター管理（`'daily'` / `'weekly'` / `'all-time'`）
- `useRankings(period)` で無限スクロールデータ取得
- `IntersectionObserver` でスクロール末尾検知 → `fetchNextPage()` 自動呼び出し
- ローディング: 10行のスケルトン
- 空: Trophy アイコン + 「{期間}のランキングはまだありません」
- エラー: `ErrorMessage` + リトライ

### 1-2. RankingTabs (`components/ranking/RankingTabs.tsx`)

3タブ切り替え UI（`FilterTabs` と同じパターン）:

| タブ | 値 | ラベル |
|---|---|---|
| デイリー | `'daily'` | デイリー |
| ウィークリー | `'weekly'` | ウィークリー |
| 累計 | `'all-time'` | 累計 |

- アクティブタブ: グラデーション背景 + 白テキスト
- 非アクティブ: グレーテキスト

### 1-3. RankingCard (`components/ranking/RankingCard.tsx`)

各ランキングエントリを表示するカードコンポーネント。

構成:
- **順位**: 1位 🥇 / 2位 🥈 / 3位 🥉（メダル絵文字）、4位以降は数字
- **ユーザー**: `UserAvatar`（トップ3は `h-10 w-10` 大サイズ）+ ユーザー名（`Link → /profile/{id}`）
- **あなたバッジ**: 自分のエントリに「あなた」グラデーションバッジ
- **チャレンジ**: チャレンジタイトル（1行 truncate）
- **回答プレビュー**: 回答テキスト（1行 truncate）
- **スコア**: AI 評価点 + いいね数（ハートアイコン付き）

自分のエントリ: グラデーション背景 tint + リングスタイルでハイライト

### 1-4. useRankings フック (`hooks/useRankings.ts`)

`useInfiniteQuery` ベース（`useFeed` と同パターン）:

| 期間 | エンドポイント | 内容 |
|---|---|---|
| `'daily'` | `GET /rankings/daily` | 直近24時間の回答 |
| `'weekly'` | `GET /rankings/weekly` | 直近7日間の回答 |
| `'all-time'` | `GET /rankings/all-time` | 全期間の回答 |

- `page_size: 20`
- バックエンドソート: `like_count DESC, created_at DESC`
- レスポンス: `AnswerWithDetails[]`（ユーザー情報 + チャレンジ情報含む）
- `getNextPageParam`: `pagination.has_more ? page + 1 : undefined`

---

## 新規ファイル一覧

### Hook (1)
| ファイル | 内容 |
|---|---|
| `hooks/useRankings.ts` | ランキング無限スクロール（`useInfiniteQuery`）+ 期間フィルター |

### Components (2)
| ファイル | 内容 |
|---|---|
| `components/ranking/RankingTabs.tsx` | デイリー / ウィークリー / 累計 タブ切り替え |
| `components/ranking/RankingCard.tsx` | ランキングカード（順位、メダル、ユーザー、スコア、自分ハイライト） |

### Pages (1)
| ファイル | 内容 |
|---|---|
| `app/(main)/rankings/RankingsContent.tsx` | ランキング Client Component |

### 変更 (1)
| ファイル | 変更内容 |
|---|---|
| `app/(main)/rankings/page.tsx` | プレースホルダー → `RankingsContent` |

---

## 設計パターン

- **既存パターン踏襲**: `useInfiniteQuery` + `IntersectionObserver`（フィードと同じ無限スクロール）
- **タブ切り替え**: `RankingTabs`（`FilterTabs` と同じデザイン、3タブに拡張）
- **自分ハイライト**: `useAuthStore` から `user.id` を取得し、`answer.user_id` と比較
- **メダル表示**: トップ3に絵文字メダル、それ以外は数字順位

## Build Status

- `pnpm build` 成功
- 全14ルートコンパイル済み
- 更新済み静的ルート: `/rankings`（プレースホルダー → 実装済み）
- 全プレースホルダーページが実装完了（Phase 1〜4 で全画面実装済み）
