use crate::DiscordMessage;
use futures::executor::block_on;
use meilisearch_sdk::client::*;

use std::env;
use std::ops::Add;
use meilisearch_sdk::errors::Error;

use meilisearch_sdk::key::{Action, Key, KeyBuilder};
use time::{Duration, OffsetDateTime};

pub fn insert_messages(messages: Vec<DiscordMessage>, index_name: String) {
    block_on(async move {
        create_client()
            .index(index_name.clone())
            .add_documents(&messages, None)
            .await
            .unwrap();
    });
}

pub fn create_client() -> Client {
    let url = env::var("MEILI_SEARCH_URL").unwrap_or(String::from("http://localhost:7700"));

    let master_key = env::var("MEILI_SEARCH_MASTER_KEY").unwrap_or(String::from("masterKey"));

    Client::new(url, master_key)
}

pub async fn create_key(client: Client, index: String) -> Result<Key, Error> {
    let mut key_options = KeyBuilder::new(format!("Add documents: {} API key", index));
    let duration_in_seconds =
        env::var("MEILI_SEARCH_READ_TOKEN_TIMEOUT_IN_SECONDS").unwrap_or(String::from("604800"));

    key_options
        .with_action(Action::DocumentsGet)
        .with_action(Action::Search)
        .with_action(Action::IndexesGet)
        .with_action(Action::Version)
        .with_action(Action::DumpsGet)
        .with_action(Action::SettingsUpdate)
        .with_action(Action::StatsGet)
        .with_action(Action::TasksGet)
        .with_expires_at(
            OffsetDateTime::now_utc().add(Duration::seconds(duration_in_seconds.parse().unwrap())),
        )
        .with_index(index);

    client.create_key(key_options).await
}
