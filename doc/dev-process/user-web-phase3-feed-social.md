# User Web Phase 3 - フィード + ソーシャル機能

## Overview

Phase 2（ホーム + チャレンジ）完了後、タイムライン（フィード）、いいね・コメント、ユーザープロフィール、マイページを実装した。Phase 2 で確立した `page.tsx` → `XxxContent.tsx` パターン、React Query + apiClient パターンを踏襲。新たに `useInfiniteQuery`（無限スクロール）、楽観的更新（いいね）、Sheet（コメントドロワー）、react-hook-form + zod（プロフィール編集）を導入。

---

## Step 1: フィード画面

### 1-1. FeedContent (`app/(main)/feed/FeedContent.tsx`)

フィードページの Client Component 本体。

- `FilterTabs` でフィルター管理（`'all'` / `'following'`）
- `useFeed(filter)` で無限スクロールデータ取得
- `IntersectionObserver` でスクロール末尾検知 → `fetchNextPage()` 自動呼び出し
- ローディング: 3枚のスケルトンカード
- 空: 「まだ投稿がありません」（フォロー中フィルター時は追加メッセージ）
- エラー: `ErrorMessage` + リトライ

### 1-2. FilterTabs (`components/feed/FilterTabs.tsx`)

2タブ切り替え UI:

| タブ | 値 |
|---|---|
| すべて | `'all'` |
| フォロー中 | `'following'` |

- アクティブタブ: グラデーション背景（`#667eea` → `#764ba2`）+ 白テキスト
- 非アクティブ: グレーテキスト
- 白背景カードスタイル + `shadow-sm`

### 1-3. useFeed フック (`hooks/useFeed.ts`)

- `useInfiniteQuery(['feed', filter], fetchFeedPage)`
- `GET /feed?filter={filter}&page={page}&page_size=20`
- `getNextPageParam`: `pagination.has_more ? page + 1 : undefined`
- カスタム `fetchFeedPage` 関数（`apiClient` ではなく直接 `fetch` で `ApiResponse<AnswerWithDetails[]>` をパース）

---

## Step 2: いいね・コメント

### 2-1. FeedCard (`components/feed/FeedCard.tsx`)

フィードの各投稿を表示するカードコンポーネント。

構成:
- **ヘッダー**: `UserAvatar` + ユーザー名（`Link → /profile/{id}`）+ 相対時間（`formatRelativeTime`）
- **チャレンジバッジ**: グラデーション背景 + チャレンジタイトル
- **回答テキスト**: 3行 `line-clamp-3`
- **スコアバッジ**: 点数表示（グラデーション tint 背景）
- **フッター**: `LikeButton` + コメントボタン（`MessageCircle` アイコン + コメント数）

コメントボタンクリック → `CommentSheet` を開く

### 2-2. LikeButton (`components/feed/LikeButton.tsx`)

ハートアイコンボタン:

- `is_liked === true`: 塗りつぶし赤（`fill-[#FF2D55] text-[#FF2D55]`）
- `is_liked === false`: アウトライングレー（`text-[#999999]`）
- クリック → `useToggleLike().mutate({ answerId, isLiked })`
- `framer-motion` の `whileTap: { scale: 1.3 }` アニメーション

### 2-3. useToggleLike フック (`hooks/useLike.ts`)

単一の `useMutation` で like/unlike を切り替え:

- `isLiked === true` → `DELETE /answers/{id}/like`
- `isLiked === false` → `POST /answers/{id}/like`

**楽観的更新（`onMutate`）:**
1. `['feed']` クエリキャンセル + キャッシュ保存
2. `['profile-answers']` クエリキャンセル + キャッシュ保存
3. 対象回答の `is_liked` を反転、`like_count` を ±1
4. `setQueriesData` で無限スクロールページ内の全ページを更新

**エラーロールバック（`onError`）:**
- `context.previousFeed` / `context.previousProfileAnswers` を復元

### 2-4. CommentSheet (`components/feed/CommentSheet.tsx`)

下からスライドする Sheet（`side="bottom"`, `h-[70vh]`）:

**コメント一覧:**
- `useComments(answerId, page)` で取得
- 各コメント: `UserAvatar` + ユーザー名 + 相対時間 + テキスト
- 自分のコメント: 削除ボタン（`Trash2` アイコン → `useDeleteComment`）
- 「もっと見る」ボタンでページネーション

**投稿フォーム:**
- 丸型テキスト入力 + グラデーション送信ボタン（`Send` アイコン）
- 空テキスト or 送信中 → 送信ボタン無効化
- 送信成功 → テキストクリア

### 2-5. useComments フック (`hooks/useComments.ts`)

| フック | API | 用途 |
|---|---|---|
| `useComments(answerId, page)` | `GET /answers/{id}/comments` | コメント一覧（ページネーション） |
| `useCreateComment(answerId)` | `POST /answers/{id}/comments` | コメント投稿 |
| `useDeleteComment(answerId)` | `DELETE /comments/{id}` | コメント削除 |

成功時:
- `['comments', answerId]` キャッシュ無効化
- `['feed']` キャッシュ無効化（コメント数更新のため）
- 削除時: `toast.success('コメントを削除しました')`

エラー時: `toast.error()` で通知

---

## Step 3: ユーザープロフィール画面

### 3-1. ProfileContent (`app/(main)/profile/[id]/ProfileContent.tsx`)

- `useParams<{ id: string }>()` でルートパラメータ取得
- `useUserProfile(id)` でプロフィールデータ取得
- `ProfileHeader` + `UserAnswerList` を縦配置
- ローディング / エラー状態を処理

### 3-2. ProfileHeader (`components/profile/ProfileHeader.tsx`)

プロフィールヘッダーカード:

**上部:**
- `UserAvatar`（`h-16 w-16`、大サイズ）+ ユーザー名 + bio

**統計（3列グリッド）:**
| キー | ラベル |
|---|---|
| `answer_count` | 回答 |
| `follower_count` | フォロワー |
| `following_count` | フォロー中 |

**アクションボタン:**
- 自分のプロフィール: 「プロフィール編集」ボタン → `/mypage`（`GradientButton secondary`）
- 他ユーザー: 「フォローする / フォロー中」ボタン（`GradientButton primary / secondary`）
- フォロートグル: `useFollowUser` / `useUnfollowUser`

### 3-3. UserAnswerList (`components/profile/UserAnswerList.tsx`)

ユーザーの回答一覧をページネーション表示:

- `useUserAnswers(userId, page)` で取得
- 各回答を `FeedCard` で表示（フィードと同じカード再利用）
- ページネーション: 「前へ / 次へ」ボタン + 現在ページ / 総ページ数
- 空: 「まだ回答がありません」

### 3-4. useProfile フック (`hooks/useProfile.ts`)

| フック | API | 用途 |
|---|---|---|
| `useUserProfile(userId)` | `GET /users/{id}` | プロフィール取得 |
| `useUserAnswers(userId, page)` | `GET /users/{id}/answers` | 回答一覧（`page_size: 10`） |
| `useFollowUser(userId)` | `POST /users/{id}/follow` | フォロー |
| `useUnfollowUser(userId)` | `DELETE /users/{id}/follow` | アンフォロー |

成功時: `['profile', userId]` キャッシュ無効化 + `toast.success()`

---

## Step 4: マイページ

### 4-1. MyPageContent (`app/(main)/mypage/MyPageContent.tsx`)

マイページの Client Component。セクション構成:

1. **ヘッダー**: `UserAvatar`（大）+ ユーザー名 + メールアドレス
2. **プロフィール編集**: `ProfileEditForm`（名前 + 自己紹介）
3. **統計**: `StatsSection`（学習統計 + ソーシャル統計）
4. **アカウント連携**: `SocialAccounts`（Google / Apple / LINE）
5. **ログアウト**: 赤いボタン（`useAuth().logout`）

### 4-2. ProfileEditForm (`components/mypage/ProfileEditForm.tsx`)

`react-hook-form` + `zod` バリデーション:

| フィールド | バリデーション |
|---|---|
| 名前 | 1〜50文字 |
| 自己紹介 | 最大200文字 |

- `useUpdateProfile()` で `PUT /users/{id}` を呼び出し
- 成功時: auth store の `user.name` も同期更新 + `toast.success()`
- 「保存」ボタン: `isDirty` でない場合は無効化

### 4-3. StatsSection (`components/mypage/StatsSection.tsx`)

2つの統計グループ:

**学習統計（5項目、3〜5列）:**
| アイコン | ラベル | キー | 色 |
|---|---|---|---|
| Flame | 連続日数 | `current_streak` | #FF9500 |
| Trophy | ベスト | `best_streak` | #FFD700 |
| CheckCircle2 | 本日完了 | `completed_today` | #34C759 |
| Star | 平均点 | `average_score` | #667eea |
| FileText | 累計 | `total_challenges` | #764ba2 |

**ソーシャル統計（4項目、2〜4列）:**
| アイコン | ラベル | キー | 色 |
|---|---|---|---|
| Users | フォロワー | `follower_count` | #667eea |
| UserPlus | フォロー中 | `following_count` | #34C759 |
| FileText | 回答数 | `answer_count` | #007AFF |
| Heart | 総いいね | `total_likes` | #FF2D55 |

### 4-4. SocialAccounts (`components/mypage/SocialAccounts.tsx`)

連携済みソーシャルアカウント管理:

- `useSocialAccounts()` で `GET /users/me/social-accounts` を取得
- 各アカウント: プロバイダーアイコン + 名前 + メール + 連携日時
- 「連携解除」ボタン → `useUnlinkSocial()` で `DELETE /users/me/social-accounts/{provider}`

プロバイダー表示:
| プロバイダー | 色 |
|---|---|
| Google | #4285F4 |
| Apple | #000000 |
| LINE | #06C755 |

### 4-5. useMyPage フック (`hooks/useMyPage.ts`)

| フック | API | 用途 |
|---|---|---|
| `useMyPage()` | `GET /users/me` | マイページデータ |
| `useUpdateProfile()` | `PUT /users/{id}` | プロフィール更新 |
| `useSocialAccounts()` | `GET /users/me/social-accounts` | 連携アカウント一覧 |
| `useUnlinkSocial()` | `DELETE /users/me/social-accounts/{provider}` | アカウント連携解除 |

`useUpdateProfile` 成功時:
- `['my-page']` キャッシュ無効化
- `useAuthStore.setUser()` で auth store の `user.name` / `user.avatar` も更新

---

## 型追加

### LinkedSocialAccount (`types/user.ts`)

```typescript
export interface LinkedSocialAccount {
  provider: string;
  provider_email: string | null;
  provider_name: string | null;
  linked_at: string;
}
```

---

## 新規ファイル一覧

### Hooks (5)
| ファイル | 内容 |
|---|---|
| `hooks/useFeed.ts` | フィード無限スクロール（`useInfiniteQuery`）+ フィルター |
| `hooks/useLike.ts` | いいねトグル（楽観的更新） |
| `hooks/useComments.ts` | コメント一覧・投稿・削除 |
| `hooks/useProfile.ts` | プロフィール・回答一覧・フォロー/アンフォロー |
| `hooks/useMyPage.ts` | マイページデータ・プロフィール更新・ソーシャルアカウント管理 |

### Components (9)
| ファイル | 内容 |
|---|---|
| `components/feed/FilterTabs.tsx` | すべて / フォロー中 タブ切り替え |
| `components/feed/FeedCard.tsx` | フィードカード（ユーザー、回答、スコア、いいね、コメント） |
| `components/feed/LikeButton.tsx` | ハートボタン（楽観的更新 + framer-motion） |
| `components/feed/CommentSheet.tsx` | コメントドロワー（Sheet） + 一覧 + 投稿フォーム |
| `components/profile/ProfileHeader.tsx` | プロフィールヘッダー + フォローボタン |
| `components/profile/UserAnswerList.tsx` | ユーザー回答一覧（ページネーション） |
| `components/mypage/StatsSection.tsx` | 学習統計 + ソーシャル統計 |
| `components/mypage/SocialAccounts.tsx` | アカウント連携管理 |
| `components/mypage/ProfileEditForm.tsx` | プロフィール編集フォーム |

### Pages (5)
| ファイル | 内容 |
|---|---|
| `app/(main)/feed/FeedContent.tsx` | フィード Client Component |
| `app/(main)/profile/[id]/page.tsx` | プロフィールページ（Server Component） |
| `app/(main)/profile/[id]/ProfileContent.tsx` | プロフィール Client Component |
| `app/(main)/mypage/MyPageContent.tsx` | マイページ Client Component |

### 変更 (3)
| ファイル | 変更内容 |
|---|---|
| `types/user.ts` | `LinkedSocialAccount` 型追加 |
| `app/(main)/feed/page.tsx` | プレースホルダー → `FeedContent` |
| `app/(main)/mypage/page.tsx` | プレースホルダー → `MyPageContent` |

---

## 設計パターン

- **無限スクロール**: `useInfiniteQuery` + `IntersectionObserver` で自動ページ読み込み
- **楽観的更新**: いいねトグルで即時 UI 反映（エラー時ロールバック）
- **Sheet コンポーネント**: Radix Dialog ベースの `side="bottom"` ドロワーでコメント表示
- **FeedCard 再利用**: フィード一覧とプロフィール回答一覧の両方で同じカードを使用
- **Auth Store 同期**: プロフィール更新時に auth store の `user` も更新（ヘッダーに即反映）

## Build Status

- `pnpm build` 成功
- 全14ルートコンパイル済み
- 新規動的ルート: `/profile/[id]`
- 更新済み静的ルート: `/feed`, `/mypage`（プレースホルダー → 実装済み）
