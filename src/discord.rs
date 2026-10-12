use crate::matcher::short_title;
use crate::models::{
    DiscordEmbed, DiscordEmbedAuthor, DiscordEmbedFooter, DiscordWebhookPayload, EpisodeRelease,
};
use worker::*;

pub const SPECTRO_MINT: u32 = 7268279; // #6ee7b7
pub const BOT_ICON: &str =
    "https://cdn.discordapp.com/avatars/1515769326639120586/52a3abbcf5977cf4355af9340d2819c5.png";

/// Builds Discord webhook payload with Spectro Mint embed card and zero emoji
pub fn build_embed_payload(
    releases: &[EpisodeRelease],
    timestamp_iso: &str,
) -> DiscordWebhookPayload {
    let mut desc_lines = vec![
        format!("`{timestamp_iso}`"),
        "".to_string(),
        "**New Episodes Detected**".to_string(),
        "".to_string(),
    ];

    for rel in releases {
        let title_fmt = short_title(&rel.title, 32);
        desc_lines.push(format!(
            "• **`{title_fmt}`** : `Ep {}` · *{}*",
            rel.episode, rel.source
        ));
    }

    desc_lines.push("".to_string());
    desc_lines.push("*Edge watcher triggered · Cloudflare Worker (Rust)*".to_string());

    let embed = DiscordEmbed {
        author: Some(DiscordEmbedAuthor {
            name: "Shorekeeper Anime · Edge Pulse Sentinel".to_string(),
            url: Some("https://samehadaku.li".to_string()),
            icon_url: Some(BOT_ICON.to_string()),
        }),
        title: "Anime Releases Updated".to_string(),
        description: desc_lines.join("\n"),
        color: SPECTRO_MINT,
        footer: Some(DiscordEmbedFooter {
            text: "Shorekeeper Edge Watcher · 1-Min Cloudflare Pulse".to_string(),
        }),
        timestamp: timestamp_iso.to_string(),
    };

    DiscordWebhookPayload {
        embeds: vec![embed],
    }
}

/// Dispatches embed payload to Discord webhook URL via Cloudflare Fetch API
pub async fn send_discord_webhook(webhook_url: &str, releases: &[EpisodeRelease]) -> Result<()> {
    if releases.is_empty() {
        return Ok(());
    }

    let now_utc = chrono::Utc::now().to_rfc3339();
    let payload = build_embed_payload(releases, &now_utc);
    let payload_json = serde_json::to_string(&payload)
        .map_err(|e| worker::Error::RustError(format!("JSON serialization error: {e}")))?;

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("User-Agent", "AnimePulseWorker/1.0 (Cloudflare Rust)")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(wasm_bindgen::JsValue::from_str(&payload_json)));

    let request = Request::new_with_init(webhook_url, &init)?;
    let response = Fetch::Request(request).send().await?;

    if response.status_code() < 200 || response.status_code() >= 300 {
        worker::console_error!(
            "Discord webhook dispatch failed with status: {}",
            response.status_code()
        );
        return Err(worker::Error::RustError(format!(
            "Discord webhook returned HTTP {}",
            response.status_code()
        )));
    }

    Ok(())
}
