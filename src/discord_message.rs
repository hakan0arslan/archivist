use meilisearch_sdk::document::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordMessage {
    id: Uuid,
    discord_id: String,
    user_id: String,
    user_name: String,
    content: String,
    timestamp: String,
    avatar_url: Option<String>,
}

impl DiscordMessage {
    pub fn new(
        discord_id: String,
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
        let id = Uuid::new_v4();

        DiscordMessage {
            id,
            discord_id,
            user_id,
            user_name,
            content,
            timestamp,
            avatar_url,
        }
    }
}

impl Document for DiscordMessage {
    type UIDType = Uuid;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}
