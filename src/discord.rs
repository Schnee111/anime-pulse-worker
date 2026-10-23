use crate::matcher::short_title;
use crate::models::{
    DiscordEmbed, DiscordEmbedAuthor, DiscordEmbedFooter, DiscordEmbedMedia, DiscordWebhookPayload,
    EpisodeRelease,
};
use worker::{js_sys, *};

pub const SPECTRO_MINT: u32 = 7268279; // #6ee7b7
pub const BOT_ICON: &str =
    "https://cdn.discordapp.com/avatars/1559227317958483998/b7b8f5fc455e84dbf226080c6b720778.png";

/// Builds a dedicated Discord webhook payload for a single episode release.
/// Keeping each anime in its own message ensures full-width 16:9 landscape image display
/// along with top-right portrait poster thumbnail.
pub fn build_single_release_payload(
    rel: &EpisodeRelease,
    timestamp_iso: &str,
    timestamp_unix: u64,
) -> DiscordWebhookPayload {
    let title_fmt = short_title(&rel.title, 42);
    let desc = format!(
        "<t:{timestamp_unix}:f> · <t:{timestamp_unix}:R>\n\n• **Episode {}** (Indonesian Subtitles)\n• Source: *{}*\n\n*Edge watcher triggered · Cloudflare Worker (Rust)*",
        rel.episode, rel.source
    );

    let thumbnail = rel
        .thumbnail_url
        .as_ref()
        .map(|url| DiscordEmbedMedia { url: url.clone() });

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
        thumbnail,
        image,
        footer: Some(DiscordEmbedFooter {
            text: "Shorekeeper Edge Watcher".to_string(),
        }),
        timestamp: timestamp_iso.to_string(),
    };

    DiscordWebhookPayload {
        embeds: vec![embed],
    }
}

/// Redacts the secret token segment from a Discord webhook URL for safe logging.
pub fn mask_webhook_url(url: &str) -> String {
    let trimmed = url.trim();
    if let Some(idx) = trimmed.rfind('/') {
        format!("{}/[REDACTED]", &trimmed[..idx])
    } else {
        "[REDACTED_WEBHOOK]".to_string()
    }
}

/// Dispatches each release as a separate Discord message to all configured webhooks
/// (supports comma-separated URLs for multi-channel / multi-guild broadcast).
///
/// Returns the subset that reached **at least one** target. Callers persist state
/// only for these: an advance on an undelivered release hides the episode from every
/// later tick, because `last_episode` never moves backwards.
pub async fn send_discord_webhook(
    webhook_url: &str,
    releases: &[EpisodeRelease],
) -> Result<Vec<EpisodeRelease>> {
    if releases.is_empty() {
        return Ok(Vec::new());
    }

    let targets: Vec<&str> = webhook_url
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if targets.is_empty() {
        return Err(worker::Error::RustError("No webhook URL provided".into()));
    }

    // `js_sys::Date` gives ISO-8601 without pulling in a calendar crate.
    let now = js_sys::Date::new_0();
    let now_utc = now.to_iso_string().as_string().unwrap_or_default();
    let now_unix = (now.get_time() / 1000.0) as u64;

    let mut delivered: Vec<EpisodeRelease> = Vec::new();

    for rel in releases {
        let payload = build_single_release_payload(rel, &now_utc, now_unix);
        let payload_json = serde_json::to_string(&payload)
            .map_err(|e| worker::Error::RustError(format!("JSON serialization error: {e}")))?;

        let mut rel_delivered = false;

        for (idx, target_url) in targets.iter().enumerate() {
            let headers = Headers::new();
            headers.set("Content-Type", "application/json")?;
            headers.set("User-Agent", "AnimePulseWorker/1.0 (Cloudflare Rust)")?;

            let mut init = RequestInit::new();
            init.with_method(Method::Post);
            init.with_headers(headers);
            init.with_body(Some(wasm_bindgen::JsValue::from_str(&payload_json)));

            let request = match Request::new_with_init(target_url, &init) {
                Ok(req) => req,
                Err(e) => {
                    worker::console_error!(
                        "Failed to create request for {} to webhook #{} ({}): {:?}",
                        rel.title,
                        idx + 1,
                        mask_webhook_url(target_url),
                        e
                    );
                    continue;
                }
            };

            match Fetch::Request(request).send().await {
                Ok(resp) => {
                    if resp.status_code() >= 200 && resp.status_code() < 300 {
                        rel_delivered = true;
                    } else {
                        worker::console_error!(
                            "Discord webhook dispatch failed for {} to webhook #{} ({}) with status: {}",
                            rel.title,
                            idx + 1,
                            mask_webhook_url(target_url),
                            resp.status_code()
                        );
                    }
                }
                Err(e) => {
                    worker::console_error!(
                        "Discord webhook fetch error for {} to webhook #{} ({}): {:?}",
                        rel.title,
                        idx + 1,
                        mask_webhook_url(target_url),
                        e
                    );
                }
            }
        }

        if !rel_delivered {
            // Isolated per release: one dead target must not cancel the others,
            // and the caller retries this one on the next tick because its
            // `last_episode` stays put.
            worker::console_error!(
                "Release '{}' Ep {} reached no webhook target, leaving state unadvanced",
                rel.title,
                rel.episode
            );
            continue;
        }

        delivered.push(rel.clone());
    }

    Ok(delivered)
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
            thumbnail_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx170942-KKcLfQzV57nG.jpg".into(),
            ),
            image_url: Some("https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into()),
        };

        let payload = build_single_release_payload(&rel, "2026-10-18T16:00:00Z", 1792339200);
        assert_eq!(payload.embeds.len(), 1);
        let embed = &payload.embeds[0];
        assert_eq!(embed.title, "Ao no Hako Season 2 · Episode 3");
        assert!(embed
            .description
            .contains("<t:1792339200:f> · <t:1792339200:R>"));
        assert!(embed.thumbnail.is_some());
        assert_eq!(
            embed.thumbnail.as_ref().unwrap().url,
            "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx170942-KKcLfQzV57nG.jpg"
        );
        assert!(embed.image.is_some());
        assert_eq!(
            embed.image.as_ref().unwrap().url,
            "https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg"
        );
    }

    #[test]
    fn test_mask_webhook_url() {
        assert_eq!(
            mask_webhook_url("https://discord.com/api/webhooks/12345/abcdef_secret_token"),
            "https://discord.com/api/webhooks/12345/[REDACTED]"
        );
        assert_eq!(mask_webhook_url("invalid_url"), "[REDACTED_WEBHOOK]");
    }
}
