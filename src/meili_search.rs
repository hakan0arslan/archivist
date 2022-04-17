use meilisearch_sdk::client::*;

use meilisearch_sdk::document::Document;
use meilisearch_sdk::errors::Error;
use std::env;
use std::ops::Add;

use meilisearch_sdk::key::{Action, Key, KeyBuilder};
use time::{Duration, OffsetDateTime};

pub async fn add_documents<T: Document>(documents: &[T], index_name: String) {
    if let Err(why) = create_client()
        .index(index_name)
        .add_documents(documents, Some("id"))
        .await
    {
        eprintln!("Error archiving documents: {:?}", why);
    }
}

pub fn create_client() -> Client {
    Client::new(retrieve_url(), retrieve_master_key())
}

pub async fn create_read_key(client: Client, index: String) -> Result<Key, Error> {
    let mut key_options = KeyBuilder::new(format!("Read documents: {} API key", index));
    let duration_in_seconds = env::var("MEILI_SEARCH_READ_TOKEN_TIMEOUT_IN_SECONDS")
        .unwrap_or_else(|_| String::from("604800"));

    key_options
        .with_action(Action::DocumentsGet)
        .with_action(Action::Search)
        .with_action(Action::IndexesGet)
        .with_action(Action::Version)
        .with_action(Action::DumpsGet)
        .with_action(Action::SettingsGet)
        .with_action(Action::StatsGet)
        .with_action(Action::TasksGet)
        .with_expires_at(
            OffsetDateTime::now_utc().add(Duration::seconds(duration_in_seconds.parse().unwrap())),
        )
        .with_index(index);

    client.create_key(key_options).await
}

pub fn retrieve_url() -> String {
    env::var("MEILI_SEARCH_URL").unwrap_or_else(|_| String::from("http://localhost:7700"))
}

fn retrieve_master_key() -> String {
    env::var("MEILI_SEARCH_MASTER_KEY").unwrap_or_else(|_| String::from("masterKey"))
}
