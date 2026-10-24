//! Otakudesu source adapter.
//!
//! Otakudesu exposes **no usable API**: `https://otakudesu.blog/wp-json/`
//! returns 401 (REST locked) and the `otakudesu.cloud` mirror returns 523. The
//! home page is a dead end too - 40 KB of HTML containing **zero** `/episode/`
//! links - so there is no cheap "latest releases" feed to poll like
//! Samehadaku's JSON.
//!
//! What does work is the opposite direction: every tracked entry already stores
//! its Otakudesu series URL, and the series page lists that show's episodes as
//! absolute links:
//!
//! ```html
//! <a href="https://otakudesu.blog/episode/tskd-s2-episode-4-sub-indo/">...</a>
//! ```
//!
//! So a sweep reads one page per tracked title and takes the highest episode,
//! along with its episode URL for the notification. Measured from Cloudflare
//! Worker egress: 200, ~31 KB, ~193 ms per series, no WAF in the way.
//!
//! This exists because the notifier was demonstrably missing releases: two of
//! eight sampled titles were a full episode ahead here versus what Samehadaku
//! and the Sokuja mirror could see.

use regex::Regex;
use std::sync::LazyLock;
use worker::*;

const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// Highest released episode of one series, with the URL to link in Discord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtakudesuRelease {
    pub episode: u32,
    pub url: String,
}

/// Matches an absolute Otakudesu episode link and captures its episode number.
///
/// Anchored on the `/episode/` path segment so unrelated `episode-N` text in
/// navigation or ad markup cannot be mistaken for a release.
static EPISODE_HREF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"href="(https?://otakudesu\.[a-z]+/episode/[^"]*?-episode-(\d+)[^"]*)""#).unwrap()
});

/// Compression container magic numbers.
///
/// A response body that still begins with one of these was never decoded by the
/// runtime, so it cannot be parsed as HTML. Naming the encoding turns a silent
/// "no episode links" into an actionable failure.
const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];
const ZSTD_MAGIC: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

/// Names the compression container `body` is still wrapped in, if any.
///
/// `otakudesu.blog` negotiates zstd, and Workers does not decode zstd response
/// bodies (`cloudflare/workerd#6836` is still open). Without this check the
/// parser sees binary garbage, matches no episode links, and reports the
/// misleading "page exposed no episode links" for every series in the sweep.
pub fn detect_undecoded_compression(body: &[u8]) -> Option<&'static str> {
    if body.starts_with(&ZSTD_MAGIC) {
        return Some("zstd");
    }
    if body.starts_with(&GZIP_MAGIC) {
        return Some("gzip");
    }
    None
}

/// Returns the highest episode published on a series page.
///
/// `None` when the page lists no episode links at all, which is also what a
/// maintenance page or an error body looks like - callers must treat that as
/// "no information", never as "zero episodes".
pub fn parse_latest_episode(html: &str) -> Option<OtakudesuRelease> {
    let mut best: Option<OtakudesuRelease> = None;

    for caps in EPISODE_HREF_RE.captures_iter(html) {
        let (Some(url), Some(number)) = (caps.get(1), caps.get(2)) else {
            continue;
        };
        let Ok(episode) = number.as_str().parse::<u32>() else {
            continue;
        };

        let candidate = OtakudesuRelease {
            episode,
            url: url.as_str().to_string(),
        };

        best = match best {
            Some(current) if current.episode >= candidate.episode => Some(current),
            _ => Some(candidate),
        };
    }

    best
}

/// Fetches one series page and parses its latest episode.
pub async fn fetch_series(series_url: &str) -> Result<OtakudesuRelease> {
    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let headers = Headers::new();
    headers.set("User-Agent", BROWSER_UA)?;
    headers.set("Accept", "text/html,application/xhtml+xml")?;
    // Leave Accept-Encoding unset: the origin returns a decoded body for every
    // value measured from a Worker, so the body is verified by magic number
    // below instead of by header guessing.
    init.with_headers(headers);

    let request = Request::new_with_init(series_url, &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        return Err(Error::RustError(format!(
            "otakudesu series returned HTTP {}",
            response.status_code()
        )));
    }

    // Read bytes rather than text so an undecoded body can be named instead of
    // being lossily decoded into unparseable characters.
    let body = response.bytes().await?;
    if let Some(encoding) = detect_undecoded_compression(&body) {
        return Err(Error::RustError(format!(
            "otakudesu body still {} compressed ({} bytes): the runtime did not decode content-encoding",
            encoding,
            body.len()
        )));
    }

    let html = String::from_utf8_lossy(&body);
    parse_latest_episode(&html).ok_or_else(|| {
        Error::RustError(format!(
            "otakudesu series page exposed no episode links ({} byte body)",
            body.len()
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERIES_PAGE: &str = r#"
        <div class="nav"><a href="https://otakudesu.blog/anime-list/">Anime List</a></div>
        <div class="episodelist">
          <ul>
            <li><a href="https://otakudesu.blog/episode/tskd-s2-episode-1-sub-indo/">Episode 1</a></li>
            <li><a href="https://otakudesu.blog/episode/tskd-s2-episode-2-sub-indo/">Episode 2</a></li>
            <li><a href="https://otakudesu.blog/episode/tskd-s2-episode-3-sub-indo/">Episode 3</a></li>
            <li><a href="https://otakudesu.blog/episode/tskd-s2-episode-4-sub-indo/">Episode 4</a></li>
          </ul>
        </div>
    "#;

    #[test]
    fn picks_the_highest_episode_and_its_url() {
        let release = parse_latest_episode(SERIES_PAGE).expect("episode expected");
        assert_eq!(release.episode, 4);
        assert_eq!(
            release.url,
            "https://otakudesu.blog/episode/tskd-s2-episode-4-sub-indo/"
        );
    }

    #[test]
    fn handles_double_digit_episodes() {
        let html = r#"<a href="https://otakudesu.blog/episode/x-episode-9-sub-indo/">9</a>
                      <a href="https://otakudesu.blog/episode/x-episode-12-sub-indo/">12</a>"#;
        assert_eq!(parse_latest_episode(html).unwrap().episode, 12);
    }

    #[test]
    fn page_without_episodes_returns_none() {
        assert!(parse_latest_episode("<html><body>no episodes here</body></html>").is_none());
    }

    #[test]
    fn ignores_episode_text_outside_episode_paths() {
        // A stray "episode-99" that is not an /episode/ link must not win.
        let html = r#"
            <span data-next="episode-99">next</span>
            <a href="https://otakudesu.blog/episode/y-episode-2-sub-indo/">2</a>
        "#;
        assert_eq!(parse_latest_episode(html).unwrap().episode, 2);
    }

    #[test]
    fn ignores_links_from_other_hosts() {
        let html = r#"<a href="https://example.test/episode/z-episode-77-sub-indo/">x</a>"#;
        assert!(parse_latest_episode(html).is_none());
    }

    /// Regression for issue #35: a body that still carries the zstd frame magic
    /// must be reported as undecoded compression, because passing it to the
    /// episode parser yields the misleading "no episode links" for every series.
    #[test]
    fn names_undecoded_zstd() {
        // Real zstd frame header from an otakudesu response: magic + frame header.
        let body = [0x28, 0xb5, 0x2f, 0xfd, 0x24, 0x08, 0x01, 0x00];
        assert_eq!(detect_undecoded_compression(&body), Some("zstd"));
    }

    /// Regression for issue #35: gzip is named too, so a runtime that fails to
    /// decode it cannot masquerade as an empty series page.
    #[test]
    fn names_undecoded_gzip() {
        let body = [0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_undecoded_compression(&body), Some("gzip"));
    }

    /// Plain HTML must never be mistaken for a compressed body, otherwise the
    /// fix would reject the responses it is supposed to accept.
    #[test]
    fn plain_html_is_not_flagged_as_compressed() {
        let html = SERIES_PAGE.as_bytes();
        assert_eq!(detect_undecoded_compression(html), None);
        // Short bodies must not panic on the slice comparison.
        assert_eq!(detect_undecoded_compression(b""), None);
        assert_eq!(detect_undecoded_compression(b"<h"), None);
    }
}
