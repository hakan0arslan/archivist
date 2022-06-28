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
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "Your messages will be archived with batches. You can check status of archive with sent token.")
                .await
            {
                eprintln!("Error sending batch information message: {:?}", why);
            }
            send_token_to_channel(&ctx, &msg).await;
            let mut messages = msg.channel_id.messages_iter(&ctx).boxed();

            let index_name = prepare_index_name(&ctx, &msg).await;

            let mut discord_messages: Vec<DiscordMessage> = Vec::new();

            let mut counter = 0;
            while let Some(message_result) = messages.next().await {
                match message_result {
                    Ok(message) => {
                        if counter < 100 {
                            discord_messages.push(DiscordMessage::new(
                                message.id.to_string(),
                                message.author.id.to_string(),
                                message.author.name.to_string(),
                                message.content,
                                message.timestamp.to_string(),
                                message.author.avatar,
                                message.edited_timestamp
                            ));
                            counter += 1;
                        } else {
                            meili_search::add_documents(
                                discord_messages.as_slice(),
                                index_name.clone(),
                            ).await;

                            discord_messages.clear();
                            counter = 0;
                        }
                    }
                    Err(error) => eprintln!("Error while getting message: {}", error),
                }
            }
            if !discord_messages.is_empty() {
                meili_search::add_documents(discord_messages.as_slice(), index_name.clone()).await;
            }

            let url = meili_search::retrieve_url();

            if let Err(why) = msg.channel_id
                .say(&ctx.http, format!("Messages archived from this channel check {} with provided token type !token to renew token", url))
                .await {
                eprintln!("Error sending final message: {:?}", why);
            }
        } else if msg.content == "!token" {
            send_token_to_channel(&ctx, &msg).await;
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn prepare_index_name(ctx: &Context, msg: &Message) -> String {
    let channel_name = msg
        .channel_id
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .name;
    let index_name = format!("{}_{}", channel_name, msg.channel_id);
    index_name
}

async fn send_token_to_channel(ctx: &Context, msg: &Message) {
    let index_name = prepare_index_name(ctx, msg).await;
    let key = meili_search::create_read_key(meili_search::create_client(), index_name).await;

    let extracted_key = match key {
        Ok(key) => key.key,
        _ => "Probably no authentication required or master key is wrong".to_string(),
    };
    let url = meili_search::retrieve_url();

    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            format!("Here is your token for {} ```{}```", url, extracted_key),
        )
        .await
    {
        eprintln!("Error sending message: {:?}", why);
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
