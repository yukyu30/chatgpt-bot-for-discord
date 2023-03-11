## ChatGPT を利用した Discord Bot

### 設定

`Secrets.toml`にトークンをセットしてください。

```
DISCORD_TOKEN="xxxxxxx"
CHAT_GPT_TOKEN="xxxxxxx"
```

### 📦 shuttleのセットアップ
1. shuttleのインストール
    ```bash
    cargo binstall cargo-shuttle
    ```
2. shuttleへログイン
    ```bash
    cargo shuttle login
    ```
3. shuttleのapi keyの取得
  ログインしたのちapi keyが付与されたのコマンドをコピーして実行し、認証を行います
    <img width="737" alt="image" src="https://user-images.githubusercontent.com/61819079/224472053-df9e85e0-d443-45c7-b1f4-12bf28e0a8d8.png">
    
    ```bash
    cargo shuttle login --api-key <your_api_key>
    ```
4. プロジェクト作成を行います
    ```bash
    cargo shuttle project new
    ```
### 🤖 開発環境での実行
```
cargo shuttle run
```

### 🚀 デプロイ

```
cargo shuttle deploy
```
### 参考
[shuttle Docs](https://docs.shuttle.rs/introduction/welcome)
