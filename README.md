# timestamp-search
## ディレクトリ構造
### frontend
Webサイトとして表示するフロントエンド側。TypeScript、React

### applications/server
・RESTful API Server
・axum

#### presentation
- RESTfulAPIのroute設定

### domain
使うdomain情報
- entities
- value_objects
- services
- repositories
    - insidevideo 自サービス内部のビデオ情報へのアクセスに関連するリポジトリ CRUD
    - timestamp 自サービス内部のtimestamp情報へのアクセスに関連するリポジトリ CRUD

#### infra
- meilisearch
- youtube_api

### shared
主にプロジェクト全体で想定されている
#### errros
エラーハンドリング用の型クラス
#### types
特定の型クラスのマクロ

## LICENSE
MIT