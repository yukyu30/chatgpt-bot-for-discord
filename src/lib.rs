mod chatgpt;

use anyhow::Context as _;
use chatgpt::get_gpt_response;
use chatgpt::RequestMessage;
use regex::Regex;
use serenity::async_trait;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use shuttle_secrets::SecretStore;

struct Bot {
    gpt_token: String,
    client: reqwest::Client,
}

fn is_user(author: &User) -> bool {
    return !author.bot;
}

fn is_inclued_bot_mention(ctx: &Context, message: &Message) -> bool {
    return message
        .mentions
        .iter()
        .any(|user| user.id == ctx.cache.current_user_id());
}

fn build_json(messages: Vec<Message>) -> Vec<RequestMessage<'static>> {
    let mention_regexp = Regex::new(r"<@(\d+)>").unwrap();
    return messages
        .iter()
        .rev()
        .map(|message| {
            let content = mention_regexp.replace_all(&message.content, "").to_string();

            let role = match is_user(&message.author) {
                true => "user",
                _ => "assistant",
            };
            RequestMessage { role, content }
        })
        .collect();
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, message: Message) {
        if is_inclued_bot_mention(&ctx, &message) && is_user(&message.author) {
            let channel_id = message.channel_id;
            let messages = match channel_id.messages(&ctx.http, |m| m.limit(100)).await {
                Ok(messages) => messages,
                Err(e) => {
                    println!("Error fetching messages: {}", e);
                    return;
                }
            };

            let requset_body: Vec<RequestMessage> = build_json(messages);
            let _typing = match message.channel_id.start_typing(&ctx.http) {
                Ok(typing) => typing,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };
            let gpt_message =
                match get_gpt_response(requset_body, &self.gpt_token, &self.client).await {
                    Ok(text) => text,
                    Err(e) => {
                        println!("Error: {}", e);
                        return;
                    }
                };
            let response = MessageBuilder::new()
                .mention(&message.author)
                .push(&gpt_message)
                .build();
            if let Err(why) = message.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }

    #[cfg(feature = "cache")]
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("cache ready");
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Secrets.tomlからトークンを取得する
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let gpt_token = secret_store
        .get("CHATGPT_TOKEN")
        .context("'CHATGPT_TOKEN' was not found")?;

    let client = get_client(&discord_token, &gpt_token).await;

    Ok(client)
}

pub async fn get_client(discord_token: &str, gpt_token: &str) -> serenity::Client {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    serenity::Client::builder(discord_token, intents)
        .event_handler(Bot {
            gpt_token: gpt_token.to_owned(),
            client: reqwest::Client::new(),
        })
        .await
        .expect("Err creating client")
}
