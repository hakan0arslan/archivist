use chrono::{DateTime, Utc};
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
        edited: Option<DateTime<Utc>>,
    ) -> DiscordMessage {
        let mut avatar_url = None;

        if let Some(avatar) = avatar_id {
            avatar_url = Some(format!(
                "https://cdn.discordapp.com/avatars/{}/{}",
                user_id, avatar
            ));
        }
        // if we create new uuid every time it archives same messages multiple times
        // uuid v5 makes it possible to create repeatable uuid
        // this way we only archive edited messages without overwriting non edited messages
        let mut base_id = discord_id.clone();

        if let Some(edit_date) = edited {
            base_id = format!("{}_{}", base_id, edit_date)
        }

        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, base_id.as_bytes());

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
