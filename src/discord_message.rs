use meilisearch_sdk::document::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordMessage {
    id: String,
    user_id: String,
    user_name: String,
    content: String,
    timestamp: String,
    avatar_url: Option<String>,
}

impl DiscordMessage {
    pub fn new(
        id: String,
        user_id: String,
        user_name: String,
        content: String,
        timestamp: String,
        avatar_id: Option<String>,
    ) -> DiscordMessage {
        let mut avatar_url = None;

        if let Some(avatar) = avatar_id {
            avatar_url = Some(format!(
                "https://cdn.discordapp.com/avatars/{}/{}",
                user_id, avatar
            ));
        }

        DiscordMessage {
            id,
            user_id,
            user_name,
            content,
            timestamp,
            avatar_url,
        }
    }
}

impl Document for DiscordMessage {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}
