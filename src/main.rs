extern crate core;

mod discord_message;
mod meili_search;

use dotenv::dotenv;
use std::env;

use crate::discord_message::DiscordMessage;
use serenity::futures::StreamExt;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!archive" {
            let mut messages = msg.channel_id.messages_iter(&ctx).boxed();

            let mut discord_messages: Vec<DiscordMessage> = Vec::new();
            while let Some(message_result) = messages.next().await {
                match message_result {
                    Ok(message) => discord_messages.push(DiscordMessage::new(
                        message.id.to_string(),
                        message.author.id.to_string(),
                        message.author.name.to_string(),
                        message.content,
                        message.timestamp.to_string(),
                        message.author.avatar,
                    )),
                    Err(error) => eprintln!("Error while getting message: {}", error),
                }
            }
            let channel_name = msg
                .channel_id
                .to_channel(&ctx)
                .await
                .unwrap()
                .guild()
                .unwrap()
                .name;

            let index_name = format!("{}_{}", channel_name, msg.channel_id.to_string());
            meili_search::insert_messages(discord_messages, index_name.clone());

            let key = meili_search::create_key(meili_search::create_client(), index_name).await;

            let extracted_key = match key {
                Ok(key) => key.key,
                _ => "Probably no authentication required or master key is wrong".to_string(),
            };

            let url = env::var("MEILI_SEARCH_URL").unwrap_or("http://localhost:7700".to_string());

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Messages archived from this channel check {} with token ```{}``` type !token to renew token", url, extracted_key)).await {
                eprintln!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!token" {
            let channel_name = msg
                .channel_id
                .to_channel(&ctx)
                .await
                .unwrap()
                .guild()
                .unwrap()
                .name;
            let index_name = format!("{}_{}", channel_name, msg.channel_id.to_string());
            let key = meili_search::create_key(meili_search::create_client(), index_name)
                .await;

            let extracted_key = match key {
                Ok(key) => key.key,
                _ => "Probably no authentication required or master key is wrong".to_string(),
            };

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Here is your token ```{}```", extracted_key))
                .await
            {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
