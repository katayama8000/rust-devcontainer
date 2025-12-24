# AWS Lambda Rust プロジェクト

RustでAWS Lambda関数を実装するプロジェクトです。

## 前提条件

- Docker / VS Code Dev Container
- AWS アカウントと認証情報

## Dev Container セットアップ

このプロジェクトはDev Containerで開発します。必要なツールは自動的にインストールされます：

- Rust (1.90.0)
- Cargo Lambda
- Zig (0.13.0) - クロスコンパイル用
- AWS CLI
- 各種開発ツール (rustfmt, clippy, wasm-pack)

### 初回起動時

1. VS Codeでプロジェクトを開く
2. "Reopen in Container"を選択
3. コンテナのビルドとセットアップが自動実行されます

## AWS認証情報の設定

Lambda関数をデプロイするには、AWS認証情報が必要です。

```bash
aws configure --profile personal
```

以下の情報を入力：
- AWS Access Key ID
- AWS Secret Access Key
- Default region name (例: `ap-northeast-1`)
- Default output format (例: `json`)

## ビルド

### 開発ビルド

```bash
cargo lambda build
```

### 本番ビルド (最適化あり)

```bash
cargo lambda build --release
```

詳細は[Cargo Lambda ドキュメント](https://www.cargo-lambda.info/commands/build.html)を参照。

## テスト

### ユニットテスト

```bash
cargo test
```

### ローカルでの統合テスト

1. ローカルサーバーを起動:

```bash
cargo lambda watch
```

2. 別のターミナルで関数を呼び出し:

サンプルイベントを使用:
```bash
cargo lambda invoke --data-example apigw-request
```

カスタムJSONファイルを使用:
```bash
cargo lambda invoke --data-file ./data.json
```

HTTPイベントの場合、curlで直接呼び出し:
```bash
curl http://localhost:9000
```

詳細は以下を参照：
- [watchコマンド](https://www.cargo-lambda.info/commands/watch.html)
- [invokeコマンド](https://www.cargo-lambda.info/commands/invoke.html)

## デプロイ

### 前提条件

- AWS認証情報が設定済み
- Lambda実行用のIAMロールが作成済み

### デプロイコマンド

```bash
cargo lambda deploy --profile personal --iam-role arn:aws:iam::YOUR_ACCOUNT_ID:role/YOUR_LAMBDA_ROLE
```

#### 初回デプロイ時

初回デプロイでは、cargo lambdaが自動的にIAMロールを作成しようとしますが、失敗する場合があります。その場合、エラーメッセージに表示されるロールARNを使用してください：

```bash
# エラーメッセージに表示されたロールARNを使用
cargo lambda deploy --profile personal --iam-role arn:aws:iam::YOUR_ACCOUNT_ID:role/cargo-lambda-role-xxxxx
```

詳細は[Cargo Lambda ドキュメント](https://www.cargo-lambda.info/commands/deploy.html)を参照。

## トラブルシューティング

### AWS認証エラー

AWS認証情報が正しく設定されているか確認:
```bash
aws sts get-caller-identity --profile personal
```


## 関連リンク

- [Cargo Lambda](https://www.cargo-lambda.info/)
- [AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- [Rust](https://www.rust-lang.org/)
