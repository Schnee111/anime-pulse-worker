use crate::matcher::short_title;
use crate::models::{
    DiscordEmbed, DiscordEmbedAuthor, DiscordEmbedFooter, DiscordEmbedMedia, DiscordWebhookPayload,
    EpisodeRelease,
};
use worker::*;

pub const SPECTRO_MINT: u32 = 7268279; // #6ee7b7
pub const BOT_ICON: &str =
    "https://cdn.discordapp.com/avatars/1515769326639120586/52a3abbcf5977cf4355af9340d2819c5.png";

/// Builds a dedicated Discord webhook payload for a single episode release.
/// Keeping each anime in its own message ensures full-width 16:9 landscape image display.
pub fn build_single_release_payload(
    rel: &EpisodeRelease,
    timestamp_iso: &str,
) -> DiscordWebhookPayload {
    let title_fmt = short_title(&rel.title, 42);
    let desc = format!(
        "`{timestamp_iso}`\n\n• **Episode {}** Subtitle Indonesia\n• Sumber: *{}*\n\n*Edge watcher triggered · Cloudflare Worker (Rust)*",
        rel.episode, rel.source
    );

    let image = rel
        .image_url
        .as_ref()
        .map(|url| DiscordEmbedMedia { url: url.clone() });

    let embed = DiscordEmbed {
        author: Some(DiscordEmbedAuthor {
            name: "Shorekeeper Anime · Edge Pulse Sentinel".to_string(),
            url: Some("https://samehadaku.li".to_string()),
            icon_url: Some(BOT_ICON.to_string()),
        }),
        title: format!("{title_fmt} · Episode {}", rel.episode),
        url: Some(rel.url.clone()),
        description: desc,
        color: SPECTRO_MINT,
        image,
        footer: Some(DiscordEmbedFooter {
            text: "Shorekeeper Edge Watcher · 1-Min Cloudflare Pulse".to_string(),
        }),
        timestamp: timestamp_iso.to_string(),
    };

    DiscordWebhookPayload {
        embeds: vec![embed],
    }
}

/// Dispatches each release as a separate Discord message so each has its own clean embed & 16:9 image
pub async fn send_discord_webhook(webhook_url: &str, releases: &[EpisodeRelease]) -> Result<()> {
    if releases.is_empty() {
        return Ok(());
    }

    let now_utc = chrono::Utc::now().to_rfc3339();

    for rel in releases {
        let payload = build_single_release_payload(rel, &now_utc);
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
                "Discord webhook dispatch failed for {} with status: {}",
                rel.title,
                response.status_code()
            );
            return Err(worker::Error::RustError(format!(
                "Discord webhook returned HTTP {}",
                response.status_code()
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_single_release_payload() {
        let rel = EpisodeRelease {
            anime_id: "ao-no-hako-s2".into(),
            title: "Ao no Hako Season 2".into(),
            episode: 3,
            source: "Samehadaku API".into(),
            url: "https://samehadaku.li/ao-no-hako-s2-episode-3/".into(),
            image_url: Some("https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into()),
        };

        let payload = build_single_release_payload(&rel, "2026-10-18T16:00:00Z");
        assert_eq!(payload.embeds.len(), 1);
        let embed = &payload.embeds[0];
        assert_eq!(embed.title, "Ao no Hako Season 2 · Episode 3");
        assert!(embed.image.is_some());
        assert_eq!(
            embed.image.as_ref().unwrap().url,
            "https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg"
        );
    }
}
