# Git: 過去のコミットメッセージを変更する方法

## ユースケース

push 済みのコミットメッセージを修正したい場合（例: `[backend]implement ph1 step2` → `[ios]implement ph1 step2`）

---

## 手順

### 1. 対象コミットを確認

```bash
git log --oneline -10
```

出力例:
```
c78d0b4 [android]implement ph1 step3
47f7ba6 [backend]implement ph1 step2    ← これを変更したい
2d3340e [backend]implement ph1 step1
```

### 2. rebase で対象コミットのメッセージを書き換え

対象コミットが HEAD から N 個前の場合、`HEAD~N` を指定する。

**1コマンドで実行する場合:**

```bash
GIT_SEQUENCE_EDITOR="sed -i '' 's/^pick <commit-hash>/reword <commit-hash>/'" \
GIT_EDITOR="sed -i '' 's/<旧メッセージ>/<新メッセージ>/'" \
git rebase -i HEAD~N
```

実例（HEAD~2 = 2つ前まで対象）:

```bash
GIT_SEQUENCE_EDITOR="sed -i '' 's/^pick 47f7ba6/reword 47f7ba6/'" \
GIT_EDITOR="sed -i '' 's/\[backend\]implement ph1 step2/[ios]implement ph1 step2/'" \
git rebase -i HEAD~2
```

**対話的に実行する場合:**

```bash
git rebase -i HEAD~2
```

エディタが開いたら、対象コミットの `pick` を `reword` に変更して保存。
次にコミットメッセージ編集画面が開くので、メッセージを修正して保存。

### 3. 変更を確認

```bash
git log --oneline -5
```

### 4. リモートに反映（force push）

```bash
git push --force
```

---

## 注意事項

- `rebase` はコミットハッシュが変わる。対象コミット以降の全コミットが新しいハッシュになる
- チームで作業している場合、`--force` push は他メンバーに影響するため事前に共有すること
- `--force-with-lease` を使うとより安全（他者の push を上書きしない）:
  ```bash
  git push --force-with-lease
  ```

---

## 直前のコミットだけ変更する場合（簡易版）

直前（HEAD）のコミットメッセージだけ変更したい場合は rebase 不要:

```bash
git commit --amend -m "新しいメッセージ"
git push --force
```

---

## macOS vs Linux の sed 差異

| OS | sed コマンド |
|----|-------------|
| macOS | `sed -i '' 's/old/new/'` （空文字列 `''` が必要） |
| Linux | `sed -i 's/old/new/'` （`''` 不要） |
