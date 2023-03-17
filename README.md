# ChatGPT を利用した Discord Bot
[技術記事](https://zenn.dev/yu_9/articles/737aca68c7fcd8)書きました

## 必要なトークン、API Keyの取得

このボットの実行には以下のトークンが必要です.
- Discord botのToken
- OpenAIのAPIキー

`Secrets.toml`にトークンをセットしてください。

```
DISCORD_TOKEN="xxxxxxx"
CHATGPT_TOKEN="xxxxxxx"
```

### OpenAIのAPIキー取得方法
[OpenAI API keys](https://platform.openai.com/account/api-keys)へアクセスして`Create new secret key`をクリックしてAPIキー発行します。
[Usage limits](https://platform.openai.com/account/billing/limits)で使用量の上限を設定しておくことを推奨します。

### Discord botの作成とトークン取得
1. [Discord Developer Portal](https://discord.com/developers/applications)へアクセスし、`New Application`をクリックし、アプリケーションを作成します
2. 作成したアプリケーションからBotのページに移動して、`Add Bot`をクリックし、Botを作成します。
3. TOKENの下にある`Copy`をクリックして、Tokenを取得します

## Botの設定と導入
### 設定
1. 初期設定では`PUBLIC BOT`がONになっているので、OFFにします
2. `MESSAGE CONTENT INTENT`をONにして、メッセージを取得できるようにします
<img width="500" alt="image" src="https://user-images.githubusercontent.com/61819079/224473934-f3d931ac-edea-4f96-8b39-d1bf49985ce5.png">

### サーバーへの導入
1. OAuth2 > URL URL Generator へアクセスします
2. SCOPESの`Bot`にチェックを入れます
3. BOT PERMISSIONSの`Send Messages`,`Read Message History`にチェックを入れます
<img width="1686" alt="image" src="https://user-images.githubusercontent.com/61819079/224474219-bae20e90-6cb4-461f-a73d-6dee5e463247.png">

4. ページの一番下にある、GENERATED URLをコピーしてブラウザに貼り付けます
5. 案内に従いBotをサーバーへ導入します

## Botを動かす
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
    
    `Cargo.toml`の`name`の項目を**任意の文字列に書き換えます。**
    shuttleで既に使われているnameと重複した場合、`a project with the same name already exists`と表示されます。
    https://github.com/yukyu30/chatgpt-bot-for-discord/blob/83ca21adbd79033e1449efca26f0b49265ef8c71/Cargo.toml#L1-L2
    ```bash
    cargo shuttle project new
    ```
    
### 🤖 開発環境での実行
```
cargo shuttle run
```

### 🚀 デプロイ
デフォルトでは30分アクセスがないとスリープモードになるため、`--idle-minutes 0`のオプションをつけます

```
cargo shuttle deploy --idle-minutes 0
```
以下のような表示がされたらデプロイ成功です👏
```
Service Name:  <project name>
Deployment ID:  xxxxxxxxx-1234-6789-1234-xxxxxxxxxx
Status:        running
Last Updated:  2023-03-11T10:08:18Z
URI:           <app_url>
```

### Botを止める
```
cargo shuttle stop
```
---
### 参考
[shuttle Docs](https://docs.shuttle.rs/introduction/welcome)

### 免責事項
本プログラムに起因するトラブル・損害・損失につきましても、 当方は一切責任を負いません。
