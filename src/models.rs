use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpTitle {
    pub rendered: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamehadakuPost {
    pub id: u64,
    pub date: String,
    pub title: WpTitle,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackedAnime {
    pub id: String,
    pub title: String,
    pub last_episode: u32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpisodeRelease {
    pub anime_id: String,
    pub title: String,
    pub episode: u32,
    pub source: String,
    pub url: String,
}

// Discord Webhook structs
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordEmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordEmbedFooter {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordEmbed {
    pub author: Option<DiscordEmbedAuthor>,
    pub title: String,
    pub description: String,
    pub color: u32,
    pub footer: Option<DiscordEmbedFooter>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordWebhookPayload {
    pub embeds: Vec<DiscordEmbed>,
}
