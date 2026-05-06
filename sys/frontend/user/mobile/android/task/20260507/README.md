# Android Gradle / KSP / Hilt 障害調査メモ

作成日: 2026-05-07
対象プロジェクト: `/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android`

## 1. 事象

IDE / Gradle Sync で以下のエラーが発生した。

```text
Unable to load class 'com.google.devtools.ksp.gradle.KspTaskJvm'
com.google.devtools.ksp.gradle.KspTaskJvm
```

再現時に Gradle CLI で確認できた本質的なエラーは次の通り。

```text
The KSP plugin was detected to be applied but its task class could not be found.

This is an indicator that the Hilt Gradle Plugin is using a different class loader because
it was declared at the root while KSP was declared in a sub-project.
```

## 2. なぜこのエラーが起きたか

このエラーは、単純な「依存キャッシュ破損」よりも、**Gradle プラグインの読み込みスコープとバージョン整合性の崩れ**で発生した可能性が高い。

今回の構成では、Hilt と KSP が両方使われているが、Hilt プラグインが KSP のタスク型 `com.google.devtools.ksp.gradle.KspTaskJvm` を参照するタイミングで、そのクラスを同じクラスローダから見つけられなかった。

つまり、現象としては「KSP のタスククラスが存在しない」のではなく、**Hilt 側から見えるクラスパスに KSP タスククラスが乗っていなかった**、という理解が正しい。

## 3. Root cause（主因）

### 主因 1: Hilt と KSP の Gradle プラグイン宣言スコープが揃っていなかった

調査時点では、`com.google.dagger.hilt.android` と `com.google.devtools.ksp` がルートとサブプロジェクトで混在しており、Hilt が KSP タスクを検出する際に**別クラスローダ**で扱われていた。

その結果、Hilt が KSP タスク型 `KspTaskJvm` を見つけられず、設定フェーズで落ちた。

### 主因 2: Kotlin / KSP / Serialization の版数が揃っていなかった

調査開始時の版数には以下の不整合があった。

- Kotlin Android plugin: `2.2.10`
- Kotlin Serialization plugin: `1.9.22`
- KSP plugin: `2.3.2`

この構成では、Kotlin 系プラグイン群の前提が揃っておらず、Gradle のプラグイン解決や KSP の動作条件が不安定になる。

特に KSP は Kotlin バージョンと強く結びつくため、**Kotlin のメジャー/マイナーラインと整合する KSP 版を使う必要がある**。

## 4. Related cause（関連要因・増幅要因）

### 関連要因 1: AGP 9 + Gradle 9.3.1 の新しい挙動に対し、既存の互換フラグへ依存していた

現在の環境:

- Android Gradle Plugin: `9.1.1`
- Gradle Wrapper: `9.3.1`

一方で `gradle.properties` には以下のような互換維持フラグが残っている。

- `android.builtInKotlin=false`
- `android.newDsl=false`
- `android.r8.optimizedResourceShrinking=false`

これはすぐに悪いわけではないが、プロジェクトが**新しい AGP 標準挙動へまだ完全移行できていない**ことを示している。実際、これらの一部を外すとビルドが壊れることを確認した。

### 関連要因 2: IDE メッセージが「キャッシュ破損」を第一候補として表示した

IDE の表示内容では、以下のような一般論が提示される。

- Gradle dependency cache may be corrupt
- Stop Gradle daemons
- kill Java processes

これ自体は一般的な案内として正しいが、今回のケースでは**直接原因ではなかった**。`./gradlew help --stacktrace` を実行すると、キャッシュ破損ではなく Hilt/KSP のクラスローダ問題であることが明確になった。

### 関連要因 3: 旧 DSL / 旧設定に依存したまま AGP を上げている

`android {}` ブロックや一部の古いプロパティは AGP 9 で非推奨警告が出ている。これ自体が即時原因ではないが、将来 AGP 10 以降で壊れるリスクを高める。

## 5. 調査で確認した事実

### 5.1 失敗の再現

以下のコマンドで問題を再現した。

```zsh
cd "/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android"
./gradlew --stop
./gradlew help --stacktrace --no-daemon
```

この時点では、`KspTaskJvm` が見つからないという設定フェーズエラーで停止した。

### 5.2 修正後の確認

修正後、以下のコマンドが成功した。

```zsh
cd "/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android"
./gradlew --stop
./gradlew help --stacktrace --no-daemon
./gradlew :app:kspDebugKotlin :app:compileDebugKotlin --stacktrace --no-daemon
```

確認結果:

- `help`: 成功
- `:app:kspDebugKotlin`: 成功
- `:app:compileDebugKotlin`: 成功

## 6. どう直したか

### 6.1 ルート `build.gradle.kts`

ルート側では Kotlin 系の版数のみを管理し、Hilt/KSP は置かない形に整理した。

最終状態:

- `com.android.application`: `9.1.1`
- `com.android.library`: `9.1.1`
- `org.jetbrains.kotlin.android`: `2.2.10`
- `org.jetbrains.kotlin.plugin.serialization`: `2.2.10`
- `org.jetbrains.kotlin.plugin.compose`: `2.2.10`

ポイント:

- `org.jetbrains.kotlin.plugin.serialization` を `2.2.10` に揃えた
- ルートから `com.google.dagger.hilt.android` を外した
- ルートから `com.google.devtools.ksp` を外した

### 6.2 `app/build.gradle.kts`

`app` モジュールで Hilt と KSP を**同じ plugins ブロック内で明示**した。

最終状態:

- `com.google.dagger.hilt.android` version `2.59.2`
- `com.google.devtools.ksp` version `2.2.10-2.0.2`

依存関係も以下に合わせた。

- `com.google.dagger:hilt-android:2.59.2`
- `com.google.dagger:hilt-android-compiler:2.59.2`
- `com.google.dagger:hilt-android-testing:2.59.2`

### 6.3 Kotlin DSL の小改善

`kotlinOptions` を deprecated ではない DSL に移行した。

- 変更前: `android { kotlinOptions { jvmTarget = "17" } }`
- 変更後: `kotlin { compilerOptions { jvmTarget = JvmTarget.JVM_17 } }`

### 6.4 `gradle.properties` の整理

安全に削除できる古いプロパティは除去した。

削除したもの:

- `android.defaults.buildfeatures.resvalues=true`
- `android.sdk.defaultTargetSdkToCompileSdkIfUnset=false`
- `android.enableAppCompileTimeRClass=false`
- `android.usesSdkInManifest.disallowed=false`
- `android.uniquePackageNames=false`

追加したもの:

- `android.generateSyncIssueWhenLibraryConstraintsAreEnabled=false`

## 7. 今まだ残しているもの

以下は非推奨ではあるが、現時点では外すと壊れる/追加対応が必要なため残している。

- `android.builtInKotlin=false`
- `android.newDsl=false`
- `android.r8.optimizedResourceShrinking=false`
- `android.r8.strictFullModeForKeepRules=false`

理由:

- `android.builtInKotlin=false` を外すと Kotlin extension の二重登録エラーが発生した
- `android.newDsl=false` を外すと AGP 新 DSL と既存 Kotlin Android plugin の相性で `BaseExtension` キャストエラーが出た

つまり、今のビルドは **KSP/Hilt 問題は解消済みだが、AGP 9 の新標準へ完全移行した状態ではない**。

## 8. How to fix（再発時の対処手順）

### 即効性のある対処

1. まず CLI で本当の失敗原因を確認する

```zsh
cd "/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android"
./gradlew --stop
./gradlew help --stacktrace --no-daemon
```

2. `KspTaskJvm` / Hilt / KSP 関連の文言が出たら、以下を確認する
   - Hilt と KSP が同じスコープで宣言されているか
   - Kotlin / Serialization / Compose / KSP の版数が揃っているか
   - Hilt の plugin 版と依存版が一致しているか

3. 次に KSP タスク単体で確認する

```zsh
./gradlew :app:kspDebugKotlin :app:compileDebugKotlin --stacktrace --no-daemon
```

4. IDE 側で古い状態を握っている場合だけ追加で実施

```zsh
./gradlew clean
```

### 今回の修正方針を一言でいうと

- **Hilt と KSP を同一モジュール・同一スコープに揃える**
- **Kotlin 系プラグインの版数を統一する**
- **KSP は Kotlin に対応する版を使う**

## 9. More reliability plan for future（今後の信頼性向上プラン）

### Plan A: バージョン管理ルールを明文化する

最低限、以下のルールを設ける。

- Kotlin 系 plugin は同じラインで統一する
  - `org.jetbrains.kotlin.android`
  - `org.jetbrains.kotlin.plugin.serialization`
  - `org.jetbrains.kotlin.plugin.compose`
- KSP は Kotlin 対応版を使う
- Hilt plugin 版と Hilt library 版を一致させる
- Hilt と KSP は同一スコープに宣言する

### Plan B: 依存・プラグイン版数を一元管理する

候補:

- `gradle/libs.versions.toml` を導入する
- plugin version と library version の出所を 1 箇所に寄せる

これにより、今回のような

- Kotlin `2.2.10`
- Serialization `1.9.22`
- KSP `2.3.2`

のようなズレをレビュー時に発見しやすくなる。

### Plan C: CI に「設定フェーズ確認」を追加する

少なくとも以下を CI に入れる。

```zsh
./gradlew help --stacktrace --no-daemon
./gradlew :app:kspDebugKotlin --stacktrace --no-daemon
./gradlew :app:compileDebugKotlin --stacktrace --no-daemon
```

`assembleDebug` だけでは、設定フェーズの問題が IDE 上で先に見つかる場合がある。`help` を入れることでプラグイン設定の破綻を早期検知できる。

### Plan D: AGP 9 / AGP 10 向けの段階的移行計画を作る

今後は以下を別タスクとして進めるのが望ましい。

1. `android.newDsl=false` を外せるように `build.gradle.kts` を新 DSL 対応へ移行
2. `android.builtInKotlin=false` を外せるように Kotlin plugin / AGP の構成を再整理
3. `android.r8.*` の旧フラグを外し、標準挙動に寄せる
4. `android {}` の deprecated API と周辺 DSL を更新する

### Plan E: IDE 表示を鵜呑みにせず、必ず CLI で切り分ける

今回のように IDE は「キャッシュ破損」や「Gradle daemon の異常」を最初に案内することがある。
しかし、本当に必要なのは次の順番で切り分けること。

1. `./gradlew --stop`
2. `./gradlew help --stacktrace --no-daemon`
3. スタックトレースから plugin classloader / version mismatch を確認
4. その後で必要なら `clean` や IDE 再起動

## 10. まとめ

今回の障害は、表面的には `KspTaskJvm` のクラス読み込み失敗だったが、実態は以下の複合問題だった。

- Hilt と KSP の plugin 適用スコープ不整合
- Kotlin 系 plugin 版数の不一致
- 新しい AGP/Gradle 環境と旧設定の混在

対処としては、Hilt/KSP を同一スコープに揃え、Kotlin 系バージョン整合性を取り、KSP と Hilt の版数を明示的に合わせることで解消した。

現在は KSP / Kotlin compile まで成功しており、当面のビルド障害は解消済み。
一方で AGP 9 互換フラグへの依存は残っているため、次の段階として **AGP 新 DSL への移行とバージョン管理の一元化**を進めるのが望ましい。

