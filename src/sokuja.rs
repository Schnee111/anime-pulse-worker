//! Sokuja source adapter.
//!
//! Sokuja runs on Next.js (App Router, SSR only) and exposes **no** public JSON
//! API: `/api/*` returns 404, `/wp-json/*` returns 403 with an empty body,
//! `/_next/data/*` returns 404 (no Pages Router) and the RSC flight payload is
//! not exposed. HTML parsing is therefore the only option, exactly like
//! Otakudesu.
//!
//! Egress note: `x6.sokuja.uk` rejects datacenter IP ranges (HTTP 403 with an
//! Indonesian deny page) including the VPS, but Cloudflare Worker egress is
//! allowed (HTTP 200). This module is designed to run inside the Worker.
//!
//! robots.txt for this host allows `/`, `/anime/`, `/jadwal-rilis-anime/`,
//! `/genre/`, `/season/`, `/studio/` and disallows `/api/`, `/_next/`,
//! `/safelink/`, `/kodsz/`, `/bookmark/`, `/tag/`, `/category/`, `/page/`.
//! Only allowed paths are requested below.

use regex::Regex;
use std::sync::LazyLock;
use worker::*;

pub const SOKUJA_BASE: &str = "https://x6.sokuja.uk";

/// Compiled once per isolate: these parsers are only reached on `/schedule` and
/// the Telegram fallback, but recompiling a regex per call is pure waste inside
/// WASM.
static EPISODE_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"href="/([a-z0-9][a-z0-9\-]*)-episode-(\d+)-subtitle-indonesia/""#).unwrap()
});

static SCHEDULE_TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#">(Senin|Selasa|Rabu|Kamis|Jumat|Sabtu|Minggu)<|\b(\d{2}:\d{2})\b|href="(/anime/([a-z0-9][a-z0-9\-]*)-subtitle-indonesia/)""#,
    )
    .unwrap()
});

/// A single episode row scraped from the Sokuja home page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SokujaEpisode {
    /// Human-ish title derived from the slug (dashes replaced by spaces).
    pub slug_title: String,
    pub episode: u32,
    pub url: String,
}

/// A single broadcast slot scraped from `/jadwal-rilis-anime/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SokujaScheduleRow {
    pub day: String,
    pub time: String,
    pub title: String,
    pub url: String,
}

fn browser_headers(accept: &str) -> Result<Headers> {
    let headers = Headers::new();
    headers.set(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    )?;
    headers.set("Accept", accept)?;
    headers.set("Accept-Language", "id-ID,id;q=0.9,en;q=0.8")?;
    // NOTE: never set Accept-Encoding here. Workers only transparently
    // decompress subrequest responses when the header is left unset.
    Ok(headers)
}

async fn fetch_html(path: &str) -> Result<String> {
    let url = format!("{SOKUJA_BASE}{path}");
    let headers =
        browser_headers("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);

    let req = Request::new_with_init(&url, &init)?;
    let mut resp = Fetch::Request(req).send().await?;

    if resp.status_code() != 200 {
        return Err(worker::Error::RustError(format!(
            "Sokuja {path} returned HTTP {}",
            resp.status_code()
        )));
    }

    resp.text().await
}

/// Converts a Sokuja slug into a comparable title.
/// `kyouran-reijou-nia-liston-byoujaku-...` -> `kyouran reijou nia liston byoujaku ...`
fn slug_to_title(slug: &str) -> String {
    slug.replace('-', " ")
}

/// Parses the Sokuja home page and extracts the distinct latest-episode links.
///
/// Markup shape (Next.js SSR, Tailwind classes):
/// ```html
/// <a class="line-clamp-1 ..." href="/shiotaiou-...-episode-3-subtitle-indonesia/">
///   Shiotaiou no Satou-san ga Ore ni da… Ep 3
/// </a>
/// ```
/// The anchor text is truncated, so the title is derived from the slug instead.
pub fn parse_latest_episodes(html: &str) -> Vec<SokujaEpisode> {
    let re = &*EPISODE_LINK_RE;

    let mut out: Vec<SokujaEpisode> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for caps in re.captures_iter(html) {
        let (Some(slug), Some(ep)) = (caps.get(1), caps.get(2)) else {
            continue;
        };
        let url = format!(
            "{SOKUJA_BASE}/{}-episode-{}-subtitle-indonesia/",
            slug.as_str(),
            ep.as_str()
        );
        if !seen.insert(url.clone()) {
            continue;
        }
        let Ok(episode) = ep.as_str().parse::<u32>() else {
            continue;
        };
        out.push(SokujaEpisode {
            slug_title: slug_to_title(slug.as_str()),
            episode,
            url,
        });
    }

    out
}

/// Parses `/jadwal-rilis-anime/` into ordered broadcast rows.
///
/// The page renders a day-tab nav first (Senin..Minggu with no rows attached),
/// then one section per day where each row is `LINK` immediately followed by
/// `TIME`. Tokens are walked in document order: every day token simply becomes
/// the current day, so the nav is overwritten by the real section heading before
/// the first row is emitted.
pub fn parse_schedule(html: &str) -> Vec<SokujaScheduleRow> {
    let re = &*SCHEDULE_TOKEN_RE;

    let mut rows: Vec<SokujaScheduleRow> = Vec::new();
    let mut current_day = String::new();
    let mut pending: Option<(String, String)> = None; // (title, url)

    for caps in re.captures_iter(html) {
        if let Some(day) = caps.get(1) {
            current_day = day.as_str().to_string();
        } else if let Some(time) = caps.get(2) {
            if let Some((title, url)) = pending.take() {
                rows.push(SokujaScheduleRow {
                    day: current_day.clone(),
                    time: time.as_str().to_string(),
                    title,
                    url,
                });
            }
        } else if let (Some(path), Some(slug)) = (caps.get(3), caps.get(4)) {
            pending = Some((
                slug_to_title(slug.as_str()),
                format!("{SOKUJA_BASE}{}", path.as_str()),
            ));
        }
    }

    rows
}

/// Fetches and parses the Sokuja home page latest-episode list.
pub async fn fetch_latest_episodes() -> Result<Vec<SokujaEpisode>> {
    let html = fetch_html("/").await?;
    Ok(parse_latest_episodes(&html))
}

/// Fetches and parses the Sokuja broadcast schedule.
pub async fn fetch_schedule() -> Result<Vec<SokujaScheduleRow>> {
    let html = fetch_html("/jadwal-rilis-anime/").await?;
    Ok(parse_schedule(&html))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_latest_episodes_dedupes_and_extracts() {
        let html = r#"
            <a class="x" href="/shiotaiou-no-satou-san-ga-ore-ni-dake-amai-episode-3-subtitle-indonesia/">Shiotaiou no Satou-san ga Ore ni da… Ep 3</a>
            <a class="y" href="/shiotaiou-no-satou-san-ga-ore-ni-dake-amai-episode-3-subtitle-indonesia/">Shiotaiou no Satou-san ga Ore ni da… Ep 3</a>
            <a class="z" href="/chitose-kun-wa-ramune-bin-no-naka-part-2-episode-12-subtitle-indonesia/">Chitose Ep 12</a>
            <a class="w" href="/anime/chitose-kun-wa-ramune-bin-no-naka-part-2-subtitle-indonesia/">not an episode link</a>
        "#;

        let eps = parse_latest_episodes(html);
        assert_eq!(eps.len(), 2, "duplicate href must collapse to one row");
        assert_eq!(eps[0].episode, 3);
        assert_eq!(
            eps[0].slug_title,
            "shiotaiou no satou san ga ore ni dake amai"
        );
        assert_eq!(
            eps[0].url,
            "https://x6.sokuja.uk/shiotaiou-no-satou-san-ga-ore-ni-dake-amai-episode-3-subtitle-indonesia/"
        );
        assert_eq!(eps[1].episode, 12);
    }

    #[test]
    fn test_parse_schedule_skips_tab_nav_and_pairs_link_time() {
        let html = r#"
            >Senin<>Selasa<>Rabu<>Kamis<>Jumat<>Sabtu<>Minggu<
            >Senin<
              href="/anime/doumo-suki-na-hito-ni-horegusuri-wo-irai-sareta-majo-desu-subtitle-indonesia/" 20:00
              href="/anime/psyren-subtitle-indonesia/" 22:00
            >Selasa<
              href="/anime/shiotaiou-no-satou-san-ga-ore-ni-dake-amai-subtitle-indonesia/" 21:20
        "#;

        let rows = parse_schedule(html);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].day, "Senin");
        assert_eq!(rows[0].time, "20:00");
        assert_eq!(
            rows[0].url,
            "https://x6.sokuja.uk/anime/doumo-suki-na-hito-ni-horegusuri-wo-irai-sareta-majo-desu-subtitle-indonesia/"
        );
        assert_eq!(rows[1].time, "22:00");
        assert_eq!(rows[2].day, "Selasa");
        assert_eq!(rows[2].time, "21:20");
        assert_eq!(rows[2].title, "shiotaiou no satou san ga ore ni dake amai");
    }
}
