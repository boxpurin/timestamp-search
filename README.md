# timestamp-search
## ディレクトリ構造
### frontend
Webサイトとして表示するフロントエンド側。TypeScript、React

### applications/server
・RESTful API Server 

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
- meilisearch meilisearch操作関連
- youtube_api YouTube Data API v3 アクセス用

### shared
プロジェクト全体で想定されている
細かいツールでも参照するかもしれない型
#### errors
エラーハンドリング用の型
#### types
主に値オブジェクト用のマクロ

## LICENSE
MIT