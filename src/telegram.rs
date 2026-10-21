//! Sokuja release feed via its public Telegram channel bridge.
//!
//! Sokuja exposes no JSON API, and scraping its Next.js homepage costs a 590 KB
//! download plus decode on every poll (~81 ms CPU measured on the Workers Free
//! tier, whose documented CPU ceiling is 10 ms). Conditional GET cannot help
//! either: the homepage sends no `ETag`/`Last-Modified` and is `no-store`.
//!
//! The channel preview at <https://t.me/s/sokuja> is a much cheaper signal with
//! better data:
//!
//! * 152 KB instead of 590 KB, plain server-rendered HTML, no WAF in the path
//!   (the datacenter-IP block that guards `x6.sokuja.uk` does not apply here).
//! * Posts carry the *full* title, the episode number and a direct `sokuja.net`
//!   link, where the homepage only offers ellipsis-truncated anchor text that
//!   forced titles to be derived from the URL slug.
//! * Each post carries an ISO `datetime`, and posts appear at broadcast time.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use worker::*;

/// Compiled once per isolate instead of per call; `parse_posts` runs on a
/// hot path inside WASM where recompiling six patterns per poll is waste.
static BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"tgme_widget_message_wrap").unwrap());
static TIME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"datetime="([^"]+)""#).unwrap());
static BODY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)tgme_widget_message_text[^>]*>(.*?)</div>").unwrap());
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]+>").unwrap());
static EPISODE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bEpisode\s+(\d+)\b").unwrap());
static LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"https?://sokuja\.(?:net|id|uk)/[A-Za-z0-9\-]+/?"#).unwrap());

/// Public channel preview for the Sokuja release feed.
const CHANNEL_URL: &str = "https://t.me/s/sokuja";

const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// One release announcement scraped from the channel preview.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramPost {
    /// Full series title as published, already stripped of decoration.
    pub title: String,
    pub episode: u32,
    /// Direct `sokuja.net` episode link.
    pub url: String,
    /// ISO-8601 publication stamp from the message bubble.
    pub published_at: String,
}

/// Downloads and parses the channel preview.
pub async fn fetch_posts() -> Result<Vec<TelegramPost>> {
    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let headers = Headers::new();
    headers.set("User-Agent", BROWSER_UA)?;
    headers.set("Accept", "text/html,application/xhtml+xml")?;
    init.with_headers(headers);

    let request = Request::new_with_init(CHANNEL_URL, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        return Err(Error::RustError(format!(
            "telegram preview returned HTTP {}",
            response.status_code()
        )));
    }

    let html = response.text().await?;
    Ok(parse_posts(&html))
}

/// Parses message bubbles out of a `t.me/s/<channel>` page.
///
/// Every bubble is a `tgme_widget_message_wrap` block containing a `<time>`
/// stamp and a `tgme_widget_message_text` body. The body is flattened to plain
/// text, then split on `Episode N`, which lets us recover the full title even
/// when it contains punctuation or a trailing period.
pub fn parse_posts(html: &str) -> Vec<TelegramPost> {
    let mut posts: Vec<TelegramPost> = Vec::new();
    // `split` on the marker yields the preamble first, so trailing blocks are
    // the actual messages; iterate all and rely on the body regex to filter.
    for block in BLOCK_RE.split(html) {
        let Some(body_caps) = BODY_RE.captures(block) else {
            continue;
        };

        let raw_body = TAG_RE.replace_all(&body_caps[1], " ");
        let text = collapse_whitespace(&unescape_entities(&raw_body));

        let Some(ep_caps) = EPISODE_RE.captures(&text) else {
            continue;
        };
        let Ok(episode) = ep_caps[1].parse::<u32>() else {
            continue;
        };

        let title = clean_title(&text[..ep_caps.get(0).unwrap().start()]);
        if title.is_empty() {
            continue;
        }

        let Some(url) = LINK_RE.find(block).map(|m| m.as_str().to_string()) else {
            continue;
        };

        let published_at = TIME_RE
            .captures(block)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        if posts.iter().any(|p| p.url == url) {
            continue;
        }

        posts.push(TelegramPost {
            title,
            episode,
            url,
            published_at,
        });
    }

    posts
}

/// Strips decoration (emoji, fire marks, quotes markers) that precedes the title.
fn clean_title(prefix: &str) -> String {
    prefix
        .trim()
        .trim_start_matches(|c: char| !c.is_alphanumeric())
        .trim()
        .to_string()
}

fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Minimal entity decoder for the handful of entities Telegram emits in text.
fn unescape_entities(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&mdash;", "-")
        .replace("&ndash;", "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shaped exactly like the live channel: decoration, full title with a
    /// trailing period, episode marker, then the direct link.
    const SAMPLE: &str = r#"
<div class="tgme_widget_message_wrap js-widget_message_wrap">
  <div class="tgme_widget_message">
    <time datetime="2026-10-20T15:56:33+00:00"></time>
    <div class="tgme_widget_message_text js-message_text" dir="auto">
      🔥 Cyberpunk: Edgerunners 2 Episode 10 Subtitle Indonesia 🔥
      <br/>🌟 Link Streaming &amp; Download 🌟
      <br/>https://sokuja.net/cyberpunk-edgerunners-2-episode-10-subtitle-indonesia/
    </div>
  </div>
</div>
<div class="tgme_widget_message_wrap js-widget_message_wrap">
  <div class="tgme_widget_message">
    <time datetime="2026-10-20T15:40:06+00:00"></time>
    <div class="tgme_widget_message_text js-message_text" dir="auto">
      🔥 Tsuihou sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka suru.
      Ore wa Buki dake ja Naku, Arayuru Mono ni "Kyouka Point" wo Fuyo Dekiru
      Episode 3 Subtitle Indonesia 🔥
      <br/>https://sokuja.net/tsuihou-sareta-cheat-fuyo-majutsushi-episode-3-subtitle-indonesia/
    </div>
  </div>
</div>
"#;

    #[test]
    fn parses_full_title_episode_and_link() {
        let posts = parse_posts(SAMPLE);
        assert_eq!(posts.len(), 2);

        assert_eq!(posts[0].title, "Cyberpunk: Edgerunners 2");
        assert_eq!(posts[0].episode, 10);
        assert_eq!(
            posts[0].url,
            "https://sokuja.net/cyberpunk-edgerunners-2-episode-10-subtitle-indonesia/"
        );
        assert_eq!(posts[0].published_at, "2026-10-20T15:56:33+00:00");
    }

    #[test]
    fn keeps_punctuation_and_inner_quotes_in_title() {
        let posts = parse_posts(SAMPLE);
        assert!(posts[1]
            .title
            .starts_with("Tsuihou sareta Cheat Fuyo Majutsushi"));
        assert!(posts[1].title.contains("\"Kyouka Point\""));
        assert!(posts[1].title.ends_with("Fuyo Dekiru"));
        assert_eq!(posts[1].episode, 3);
    }

    #[test]
    fn decodes_entities_and_collapses_whitespace() {
        let posts = parse_posts(SAMPLE);
        assert!(!posts[0].title.contains("  "));
    }

    #[test]
    fn blocks_without_episode_marker_are_skipped() {
        let html = r#"
<div class="tgme_widget_message_wrap">
  <div class="tgme_widget_message_text">Just an announcement, no release here</div>
</div>"#;
        assert!(parse_posts(html).is_empty());
    }
}
