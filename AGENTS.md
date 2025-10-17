# Repository Guidelines

## プロジェクト構成とモジュール整理
`frontend/` は TypeScript + React の Vite アプリで、`src/
  ` に画面コンポーネント、`styles/` に共通テーマを配置しま
す。`service-backend/` は Rust ワークスペースで、ドメイ
ン層 (`lib/domains` / `lib/types` / `lib/errors`)、ユース
ケース層 (`lib/usecase`)、インフラ層 (`lib/meilisearch` /
`lib/youtube`) が分離されています。REST API は `services/
  restful_server` が担当し、動画取得ジョブは `services/
  video_fetch` に実装されています。結合テストは `service-
  backend/tests/integration` 直下、テストユーティリティは
`service-backend/tests/utils` にまとめます。`scripts/` には
Meilisearch 用 Python ツールがあり、`docker/` と `docker-
  compose.yml` がローカルスタックを管理します。

## ビルド・テスト・開発コマンド
Rust 関連は `mise run build-all`・`mise run run-server`・
`mise run run-fetch` を利用します。品質チェックは `mise run
  fmt` と `mise run clippy`、静的検査は `mise run check` が
基本です。フロントエンドは `pnpm --dir frontend install`
を初回に実行し、`pnpm --dir frontend dev` でホットリロー
ド、`pnpm --dir frontend build` で本番ビルドを生成します。
Meilisearch を起動する際は `mise run compose-up` または軽量
テスト用途の `mise run compose-up-test` を使います。

## コーディングスタイルと命名規則
Rust は `cargo fmt` に準拠した 4 スペースインデントを厳守
し、`cargo clippy -- -D warnings` を無警告で通すことをレ
ビュー条件とします。モジュール名・ファイル名は snake_case、
型・トレイトは PascalCase、定数は SCREAMING_SNAKE_CASE を
使用してください。TypeScript は `eslint.config.js` と
`prettier` の設定に従い、関数コンポーネントとファイル名は
PascalCase、ユーティリティは camelCase、カスタムフックは
`use` プレフィックスを付与します。

## テストガイドライン
`mise run test-all` で `cargo nextest` を用いたワークスペー
ス全体テストを実行します。ユニットテストのみ確認したい場
合は `mise run run-unit-test`、結合テストは `mise run run-
  integral-test` を利用します。新規テストは `#[cfg(test)]` ブ
ロックまたは `tests/integration` 配下に追加し、`rstest` の
フィクスチャを活用してケースを網羅してください。データス
トアを伴うケースは Meilisearch を `compose-up-test` で先に
起動し、終了時に `mise run compose-remove-test` でクリーン
アップします。

## コミットとプルリクエスト指針
コミットメッセージは履歴に倣い、要約を 40 文字前後の日本語
で簡潔に書き、プレフィックスは不要です。複数変更を含む場合
は箇条書き本文で影響を説明してください。PR では目的・主要変
更点・テスト結果 (`mise run test-all` など)・関連 Issue を
記載し、UI 変更がある場合はスクリーンショットを添付します。
新しいエンドポイントやジョブを追加した際は、環境変数や必要
な `mise` タスクの更新を必ず説明してください。

## 設定と運用のヒント
`.env.*` は `mise` により自動読み込みされるため、共有が必要
なキーはテンプレート化し `README` や `scripts/` 内のコメン
トに反映します。Meilisearch のインデックス作成は `mise run
  setup-meili` を用い、実行前に Python 依存を `uv` が管理す
る点に留意してください。Docker での統合検証時は `mise run
  compose-up` 後にバックエンドを `mise run run-server` で起
動し、完了後に `mise run compose-down` でリソースを解放し
ます。