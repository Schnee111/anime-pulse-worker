pub mod auth;
pub mod discord;
pub mod ledger;
pub mod matcher;
pub mod models;
pub mod otakudesu;
pub mod profile;
pub mod sokuja;
pub mod telegram;

use models::{EpisodeRelease, SamehadakuPost, TrackedAnime};
use profile::{Stage, TickProfile};
use worker::*;

pub fn default_tracked_list() -> Vec<TrackedAnime> {
    vec![
        TrackedAnime {
            id: "koori-jouheki-s2".into(),
            title: "Koori no Jouheki Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/koori-jouheki-s2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx186497-uwPrNPphXvjP.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/a7ah5by0Ffw/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "ao-no-hako-s2".into(),
            title: "Ao no Hako Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/ao-hako-s2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx170942-KKcLfQzV57nG.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "yowaki-max-reijou".into(),
            title: "Yowaki Max Reijou nanoni, Ratsuwan Konyakusha-sama no Kake ni Notte Shimatta".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/yowaki-max-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx200455-P3XStRQMJ7Di.png".into()),
            image_url: Some("https://img.youtube.com/vi/e41RGxVwJRs/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tensei-shita-daiseijo".into(),
            title: "Tensei shita Daiseijo wa, Seijo de Aru Koto wo Hitakakusu".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/tensei-shita-daiseijo-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx187402-ReKkLwFmMV3q.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/9oBywQ403ZA/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "futsutsuka-na-akujo".into(),
            title: "Futsutsuka na Akujo dewa Gozaimasu ga".into(),
            last_episode: 11,
            url: "https://otakudesu.blog/anime/futsutsuka-akujo-gozaimasu-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx188139-1qIJfWxym8FX.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/nR_vqhD2rGM/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tempal-chikara".into(),
            title: "Tempal: Item no Chikara".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/tempal-chikara-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx212888-NhyvqPt5aIuJ.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/e1Km0FqUZl0/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "fx-senshi-kurumi".into(),
            title: "FX Senshi Kurumi-chan".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/fx-senshi-kurumi-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx206401-8cAebkI0i8BA.png".into()),
            image_url: Some("https://img.youtube.com/vi/7rxIZ3z0S4s/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "saikyosoubi".into(),
            title: "Mezametara Saikyou Soubi to Uchuusenmochi Datta node, Ikkodate Mezashite Youhei toshite Jiyuu ni Ikitai".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/saikyosoubi-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx186541-caYpLsLmbCh7.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/Z9ofT53sz2U/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "yasei-last-boss-s2".into(),
            title: "Yasei no Last Boss ga Arawareta! Season 2".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/yasei-last-bos-s2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx180523-FjvW1XByUNMK.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/KVT_ODZn9Ys/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "romelia-senki".into(),
            title: "Romelia Senki".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/romelia-senki-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx180894-o3pz4DWFm3je.png".into()),
            image_url: Some("https://img.youtube.com/vi/PxXPcoo8uGM/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "kyouran-reijou-nia-liston".into(),
            title: "Kyouran Reijou Nia Liston".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/kyouran-reijou-liston-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx206949-4HYI3YuP0eLI.png".into()),
            image_url: Some("https://img.youtube.com/vi/gxG4vntLtbk/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "doumo-suki-na-majo".into(),
            title: "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu.".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/doumo-suki-majo-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx207191-VgkIzfH92WoZ.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/hGRtocAh3iw/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "magical-explorer".into(),
            title: "Magical★Explorer".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/magic-explor-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx169581-UlAviVH36Hxi.png".into()),
            image_url: Some("https://img.youtube.com/vi/XqrBfyUNYZs/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "seitokai-ni-mo-ana-wa-aru".into(),
            title: "Seitokai ni mo Ana wa Aru!".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/seitokai-ana-aru-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx191656-xFHtxM8SUTdU.png".into()),
            image_url: Some("https://img.youtube.com/vi/VTRVLnzhl90/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "zombie-sagashitemasu".into(),
            title: "#Zombie Sagashitemasu".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/zombie-saga-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx199007-8R1HsnflnHJE.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/_0kVb8-uRSI/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "sekai-saikyou-no-majo".into(),
            title: "Sekai Saikyou no Majo, Hajimemashita".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/sekai-hajimemashita-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx211778-0MXK5HqVFBYH.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/snJZD9vxfHY/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "chitose-kun-ramune-p2".into(),
            title: "Chitose-kun wa Ramune Bin no Naka Part 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/chitose-ramune-bin-naka-p2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx180082-IZAlf06G2VVm.png".into()),
            image_url: Some("https://img.youtube.com/vi/wC7FYPFHLeQ/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tantei-wa-mou-shindeiru-s2".into(),
            title: "Tantei wa Mou, Shindeiru. Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/tante-mou-shindeiru-s2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx128712-AlVwHbrugvj6.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/MghFONe--0Q/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tsuihou-sareta-cheat".into(),
            title: "Tsuihou Sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka Suru".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/chii-fuyo-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/b207329-6VPeZIDfF4Sr.png".into()),
            image_url: Some("https://img.youtube.com/vi/BJXBr_aFF8s/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "shiotaiou-satou-san".into(),
            title: "Shiotaiou no Satou-san ga Ore ni dake Amai".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/shiotaiou-satou-amai-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx209502-C81oN8M4Smlo.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/8XgKVzuLmJw/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "hyouken-suberu-s2".into(),
            title: "Hyouken no Majutsushi ga Sekai wo Suberu Season 2".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/hyouken-suberu-s2-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx212503-sXBpRZr96c4e.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/oxkxyAcKv2g/maxresdefault.jpg".into()),
        },
        // Added late: AniList 205909, 12 episodes, already at Ep 3 on Otakudesu
        // when it joined the list, so tracking starts at Ep 4 instead of
        // replaying the backlog.
        TrackedAnime {
            id: "hotaru-no-yomeiri".into(),
            title: "Hotaru no Yomeiri".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/hotaru-yomeiri-sub-indo/".into(),
            thumbnail_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx205909-DM0fAzNQulod.jpg".into()),
            image_url: Some("https://img.youtube.com/vi/Ex9LrBK-7hk/maxresdefault.jpg".into()),
        },
    ]
}

/// Merges defaults into tracked list to seamlessly migrate legacy KV records
/// that might be missing thumbnail_url or holding outdated image_urls.
pub fn merge_tracked_defaults(tracked: &mut Vec<TrackedAnime>) {
    let defaults = default_tracked_list();
    for item in tracked.iter_mut() {
        if let Some(def) = defaults.iter().find(|d| d.id == item.id) {
            if item.thumbnail_url.is_none() {
                item.thumbnail_url = def.thumbnail_url.clone();
            }
            if item.image_url != def.image_url {
                item.image_url = def.image_url.clone();
            }
        }
    }
    for def in defaults {
        if !tracked.iter().any(|t| t.id == def.id) {
            tracked.push(def);
        }
    }
}

/// A raw release candidate before it is matched against the tracked watchlist.
pub struct ReleaseCandidate {
    pub title: String,
    pub episode: u32,
    pub url: String,
    pub source: &'static str,
    /// When set, the candidate is bound to one tracked entry by id and skips
    /// fuzzy title matching. Otakudesu candidates use this: the series URL came
    /// from that very entry, so re-deriving the title would only add risk.
    pub anime_id: Option<String>,
}

/// Source 1: Samehadaku WP-JSON REST API (fast, structured).
async fn fetch_samehadaku_candidates() -> Result<Vec<ReleaseCandidate>> {
    let fetch_url =
        "https://samehadaku.li/wp-json/wp/v2/posts?per_page=15&_fields=id,date,title,link";

    let headers = Headers::new();
    headers.set(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    )?;
    headers.set("Accept", "application/json")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);

    let req = Request::new_with_init(fetch_url, &init)?;
    let mut resp = Fetch::Request(req).send().await?;

    if resp.status_code() != 200 {
        return Err(worker::Error::RustError(format!(
            "Samehadaku API returned HTTP {}",
            resp.status_code()
        )));
    }

    let body_str = resp.text().await?;
    let posts: Vec<SamehadakuPost> = serde_json::from_str(&body_str)
        .map_err(|e| worker::Error::RustError(format!("Samehadaku JSON parse failed: {e}")))?;

    let mut out = Vec::new();
    for post in posts.iter() {
        if let Some((parsed_title, episode_num)) =
            matcher::parse_release_title(&post.title.rendered)
        {
            out.push(ReleaseCandidate {
                title: parsed_title,
                episode: episode_num,
                url: post.link.clone(),
                source: "Samehadaku API",
                anime_id: None,
            });
        }
    }
    Ok(out)
}

// Collects release candidates from every configured source and matches them
// against the tracked watchlist.
//
// A source failure is logged and skipped - one degraded upstream must never
// suppress releases reported by the other. Because `last_episode` is advanced
// as soon as a candidate matches, the same episode reported by two sources
// produces a single notification (Samehadaku is evaluated first, so its direct
// post link wins).
//
// The Sokuja line is served by the Telegram bridge on a 15 minute throttle,
// with the HTML homepage kept as a fallback for when the bridge is down.

/// Minimum gap between Telegram channel polls.
///
/// The channel page is 152 KB versus the 590 KB Sokuja homepage, but at a
/// 60 s cadence it would still dominate the CPU budget on the Workers Free
/// tier. Fifteen minutes keeps the safety net while cutting the cost ~98%;
/// Samehadaku remains the 60 s fast path.
const TELEGRAM_INTERVAL_MS: i64 = 15 * 60 * 1000;

/// KV key holding the epoch-millis stamp of the last Telegram poll.
const TELEGRAM_LAST_CHECK_KEY: &str = "telegram_last_check";

/// KV key holding the epoch-millis stamp of the last Otakudesu sweep.
const OTAKUDESU_LAST_CHECK_KEY: &str = "otakudesu_last_check";

/// How many distinct failure reasons one sweep writes to the log.
const MAX_LOGGED_FAILURES: usize = 4;

/// Period of the external scheduler that drives `/trigger` from the VPS
/// (`*/15 * * * * sleep 25 && /home/ubuntu/scripts/anime-pulse-trigger.sh`).
const EXTERNAL_TRIGGER_PERIOD_MS: i64 = 15 * 60 * 1000;

/// Minimum gap between Otakudesu series sweeps.
///
/// A sweep is one request per tracked title (22 today, capped at 5 in flight),
/// which is far more expensive than a single JSON poll, so it is throttled
/// rather than run every tick.
///
/// 14 minutes, not 15, because the sweep is now driven by an external scheduler
/// that fires every 15 minutes (`*/15 * * * * sleep 25 && ...` on the VPS). With
/// both periods equal, a trigger arriving even a second earlier than the previous
/// one would find the window closed, so honest runs would skip and the effective
/// cadence would silently halve. One minute of headroom makes every trigger due.
const OTAKUDESU_INTERVAL_MS: i64 = 14 * 60 * 1000;

// Compile-time, because this is a structural relationship between the Worker and
// the external scheduler, not a value to sanity check at runtime.
const _: () = assert!(
    OTAKUDESU_INTERVAL_MS < EXTERNAL_TRIGGER_PERIOD_MS,
    "throttle must expire before the next external trigger, or honest runs skip",
);
const _: () = assert!(
    EXTERNAL_TRIGGER_PERIOD_MS - OTAKUDESU_INTERVAL_MS >= 60_000,
    "leave at least a minute of headroom for scheduler jitter",
);

/// True when the stage guarded by `key` is due for another run.
///
/// Fails open: if the stamp is missing or unreadable we run, because a stray
/// extra fetch costs far less than a silently dead source.
async fn throttle_due(kv: &KvStore, key: &str, interval_ms: i64) -> bool {
    let now = profile::now_ms();
    match kv.get(key).text().await {
        Ok(Some(raw)) => match parse_stamp(&raw) {
            Some(last) => now.saturating_sub(last) >= interval_ms,
            None => true,
        },
        _ => true,
    }
}

/// Parses the throttle stamp written by [`mark_throttled`].
///
/// `KvStore::put` serialises a `String` through `serde_json`, so the stored
/// value comes back as `"<millis>"` with the quotes intact. Accepting both the
/// quoted and the bare form keeps the throttle from silently failing open on
/// every tick, which would re-run the stage every 60 s.
fn parse_stamp(raw: &str) -> Option<i64> {
    raw.trim().trim_matches('"').trim().parse::<i64>().ok()
}

/// Records the current time as the last run of the stage guarded by `key`.
///
/// Marked on attempt rather than success so an outage cannot escalate into a
/// retry every tick, each with its full download cost.
async fn mark_throttled(kv: &KvStore, key: &str) {
    let stamp = profile::now_ms().to_string();
    if let Ok(put) = kv.put(key, stamp) {
        if let Err(e) = put.execute().await {
            worker::console_error!("Could not persist throttle stamp {}: {:?}", key, e);
        }
    }
}

/// Result of a concurrent source sweep.
///
/// Timings are returned rather than written into a `&mut TickProfile` so the
/// sweep can run inside `join` without holding a mutable borrow.
pub struct Sources {
    pub candidates: Vec<ReleaseCandidate>,
    pub samehadaku_ms: i64,
    pub telegram_ms: i64,
}

/// Fetches every configured source, with the two independent round trips
/// overlapped.
///
/// The Samehadaku request dominates a tick (~670 ms measured) and the Telegram
/// throttle stamp is a ~100 ms KV read, so serialising them wastes most of the
/// KV latency. `join` hides it under the network call. The Telegram fetch
/// itself still waits for the stamp, because whether to fetch at all depends on
/// it, and it only runs on 1 tick in 15.
pub async fn collect_sources(kv: &KvStore) -> Sources {
    let samehadaku = async {
        let started = profile::now_ms();
        let result = fetch_samehadaku_candidates().await;
        (result, profile::now_ms().saturating_sub(started).max(0))
    };

    let throttle = async {
        let started = profile::now_ms();
        let due = throttle_due(kv, TELEGRAM_LAST_CHECK_KEY, TELEGRAM_INTERVAL_MS).await;
        (due, profile::now_ms().saturating_sub(started).max(0))
    };

    let ((samehadaku_result, samehadaku_ms), (due, stamp_ms)) =
        futures_util::future::join(samehadaku, throttle).await;

    let mut candidates: Vec<ReleaseCandidate> = Vec::new();
    match samehadaku_result {
        Ok(mut c) => candidates.append(&mut c),
        Err(e) => worker::console_error!("Source Samehadaku unavailable, skipping: {:?}", e),
    }

    let mut telegram_ms = stamp_ms;
    if due {
        let started = profile::now_ms();
        match telegram::fetch_posts().await {
            Ok(posts) => {
                mark_throttled(kv, TELEGRAM_LAST_CHECK_KEY).await;
                worker::console_log!("Telegram bridge reported {} release posts", posts.len());
                for p in posts {
                    candidates.push(ReleaseCandidate {
                        title: p.title,
                        episode: p.episode,
                        url: p.url,
                        source: "Sokuja (Telegram)",
                        anime_id: None,
                    });
                }
            }
            Err(e) => {
                mark_throttled(kv, TELEGRAM_LAST_CHECK_KEY).await;
                worker::console_error!(
                    "Telegram bridge unavailable ({}), falling back to Sokuja HTML",
                    e
                );
                match sokuja::fetch_latest_episodes().await {
                    Ok(eps) => {
                        worker::console_log!("Sokuja HTML fallback reported {} links", eps.len());
                        for e in eps {
                            candidates.push(ReleaseCandidate {
                                title: e.slug_title,
                                episode: e.episode,
                                url: e.url,
                                source: "Sokuja",
                                anime_id: None,
                            });
                        }
                    }
                    Err(e2) => {
                        worker::console_error!("Sokuja HTML fallback unavailable too: {:?}", e2)
                    }
                }
            }
        }
        telegram_ms += profile::now_ms().saturating_sub(started).max(0);
    } else {
        worker::console_log!("Telegram bridge skipped, next poll window not reached");
    }

    Sources {
        candidates,
        samehadaku_ms,
        telegram_ms,
    }
}

/// Normalises a failure message so one underlying cause occupies one log slot.
///
/// Series pages differ in size, so a raw message like "no episode links (31078
/// byte body)" would differ per title and burn every slot on the same cause.
/// Byte counts are masked; status codes are deliberately preserved, because
/// HTTP 403 and HTTP 500 are different problems.
fn group_failure_reason(reason: &str) -> String {
    let chars: Vec<char> = reason.chars().collect();
    let mut out = String::with_capacity(reason.len());
    let mut i = 0;

    while i < chars.len() {
        if !chars[i].is_ascii_digit() {
            out.push(chars[i]);
            i += 1;
            continue;
        }

        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        let tail: String = chars[i..].iter().take(5).collect();
        // Only size figures are noise; a status code must survive grouping.
        if tail.starts_with(" byte") {
            out.push('#');
        } else {
            out.extend(&chars[start..i]);
        }
    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Sweeps every tracked series page on Otakudesu.
///
/// Concurrency is capped at 5: Workers Free allows only 6 simultaneous outgoing
/// connections per invocation, so 20 series must never be fired at once. Each
/// candidate is bound to its tracked id, because the series URL came from that
/// entry and re-deriving a title would only add matching risk.
async fn sweep_otakudesu(tracked: &[TrackedAnime]) -> Vec<ReleaseCandidate> {
    use futures_util::stream::{self, StreamExt};

    let targets: Vec<(String, String, String)> = tracked
        .iter()
        .filter(|entry| entry.url.contains("otakudesu"))
        .map(|entry| (entry.id.clone(), entry.title.clone(), entry.url.clone()))
        .collect();

    if targets.is_empty() {
        return Vec::new();
    }

    let results =
        stream::iter(targets.into_iter().map(|(id, title, url)| async move {
            (id, title, otakudesu::fetch_series(&url).await)
        }))
        .buffer_unordered(5)
        .collect::<Vec<_>>()
        .await;

    let mut candidates = Vec::new();
    let mut fetched = 0usize;
    let mut failed = 0usize;
    let mut reasons: Vec<String> = Vec::new();

    for (id, title, outcome) in results {
        match outcome {
            Ok(release) => {
                fetched += 1;
                candidates.push(ReleaseCandidate {
                    title,
                    episode: release.episode,
                    url: release.url,
                    source: "Otakudesu",
                    anime_id: Some(id),
                });
            }
            Err(e) => {
                failed += 1;
                // Keep the reason, grouped and capped: a total outage used to log
                // only a count, which made a dead source look healthy. Byte
                // counts are stripped before grouping, otherwise per-series body
                // sizes vary and fill every slot with the same underlying cause.
                let reason = group_failure_reason(&e.to_string());
                if reasons.len() < MAX_LOGGED_FAILURES && !reasons.contains(&reason) {
                    reasons.push(reason);
                }
            }
        }
    }

    worker::console_log!(
        "Otakudesu sweep: {} series read, {} unavailable",
        fetched,
        failed
    );
    if !reasons.is_empty() {
        worker::console_log!("Otakudesu failure reasons: {}", reasons.join(" | "));
    }
    candidates
}

/// Matches candidates against the watchlist, advancing `last_episode`.
///
/// Pure apart from the `last_episode` mutation, so it is unit-testable without
/// network or KV. Samehadaku candidates are supplied first, and because
/// `last_episode` advances on the first match, a duplicate episode from a
/// second source produces exactly one notification.
pub fn match_releases(
    tracked: &[TrackedAnime],
    candidates: Vec<ReleaseCandidate>,
) -> Vec<EpisodeRelease> {
    // Collapse per tracked entry first, keeping the highest episode and the
    // candidate that announced it. Dedupe used to rely on `last_episode`
    // advancing on the first match, which only works while sources arrive in
    // ascending order: one inconsistent source produced two cards for a single
    // release (`Ao no Hako Season 2` Ep 3 and Ep 4 in one sweep).
    let mut best: Vec<(usize, &ReleaseCandidate)> = Vec::new();
    for cand in candidates.iter() {
        for (idx, item) in tracked.iter().enumerate() {
            let title_matches = match cand.anime_id.as_deref() {
                Some(id) => id == item.id,
                None => matcher::matches_tracked(&item.title, &cand.title),
            };
            if !title_matches || cand.episode <= item.last_episode {
                continue;
            }
            match best.iter_mut().find(|(slot, _)| *slot == idx) {
                Some(slot) => {
                    if cand.episode > slot.1.episode {
                        slot.1 = cand;
                    }
                }
                None => best.push((idx, cand)),
            }
        }
    }

    let mut detected_releases = Vec::new();
    for (idx, cand) in best {
        let item = &tracked[idx];
        // Not yet committed to KV: the caller only persists the episodes it
        // actually delivered, so a failed webhook is retried on the next tick
        // instead of being lost.
        detected_releases.push(EpisodeRelease {
            anime_id: item.id.clone(),
            title: item.title.clone(),
            episode: cand.episode,
            source: cand.source.into(),
            url: cand.url.clone(),
            thumbnail_url: item.thumbnail_url.clone(),
            image_url: item.image_url.clone(),
        });
    }

    detected_releases
}

/// Applies delivered releases to the watchlist and returns how many moved.
///
/// Delivery-then-commit: nothing is persisted until the webhook confirmed the
/// card, because an advanced `last_episode` permanently hides the episode from
/// every future tick.
pub fn apply_delivered_episodes(
    tracked: &mut [TrackedAnime],
    delivered: &[EpisodeRelease],
) -> usize {
    let mut advanced = 0;
    for rel in delivered {
        if let Some(item) = tracked
            .iter_mut()
            .find(|item| item.id == rel.anime_id && rel.episode > item.last_episode)
        {
            item.last_episode = rel.episode;
            advanced += 1;
        }
    }
    advanced
}

/// Advances `tracked` to episodes the ledger says were already delivered.
///
/// A tick works from a snapshot that can be a minute stale. When the ledger
/// refuses an episode because it is already final, that refusal is proof the
/// snapshot is behind, and writing it back unchanged would undo the delivery
/// another invocation committed. Returns how many entries moved.
fn reconcile_already_delivered(tracked: &mut [TrackedAnime], keys: &[String]) -> usize {
    let mut advanced = 0;
    for key in keys {
        let Some((anime_id, episode)) = ledger::parse_key(key) else {
            continue;
        };
        if let Some(item) = tracked.iter_mut().find(|item| item.id == anime_id) {
            if episode > item.last_episode {
                item.last_episode = episode;
                advanced += 1;
            }
        }
    }
    advanced
}

/// Announces only the releases no other invocation has already claimed.
///
/// The watchlist snapshot a tick works from can be up to a minute stale, so the
/// same episode can be matched twice by two invocations that never overlap (cron
/// at `:00`, external trigger at `:25`). The dedupe therefore cannot live in KV:
/// it has to sit in strongly consistent storage, which is the `DeliveryLedger`
/// Durable Object. See issue #43.
///
/// Ordering is claim -> send -> commit, so a webhook failure releases the claim
/// and the episode stays queued for the next tick, exactly as before.
///
/// If the ledger itself is unreachable this degrades to the old behaviour and
/// announces everything, because a duplicate card is less harmful than an episode
/// that is never delivered at all. The degradation is logged loudly every tick.
async fn deliver_unclaimed_releases(
    env: &Env,
    kv: &KvStore,
    tracked: &mut [TrackedAnime],
    releases: &[EpisodeRelease],
    webhook_url: &str,
) -> Result<usize> {
    let wanted: Vec<String> = releases
        .iter()
        .map(|rel| ledger::key(&rel.anime_id, rel.episode))
        .collect();

    let (to_send, managed_by_ledger) = match ledger::claim(env, &wanted).await {
        Ok(claim) => {
            // An episode refused as already-final was delivered by another
            // invocation, which means this snapshot is behind for that title.
            let reconciled = reconcile_already_delivered(tracked, &claim.already_final);
            if reconciled > 0 {
                worker::console_log!(
                    "Reconciled {} episode(s) already announced by another invocation",
                    reconciled
                );
            }
            if claim.granted.is_empty() {
                worker::console_log!(
                    "All {} matched releases were already announced by another invocation",
                    releases.len()
                );
                return Ok(0);
            }
            let allowed: std::collections::HashSet<&String> = claim.granted.iter().collect();
            (
                releases
                    .iter()
                    .filter(|rel| allowed.contains(&ledger::key(&rel.anime_id, rel.episode)))
                    .cloned()
                    .collect::<Vec<_>>(),
                true,
            )
        }
        Err(e) => {
            worker::console_error!(
                "Delivery ledger unavailable ({:?}); falling back to delivery without dedupe",
                e
            );
            (releases.to_vec(), false)
        }
    };

    let delivered = match discord::send_discord_webhook(webhook_url, &to_send).await {
        Ok(delivered) => delivered,
        Err(e) => {
            if managed_by_ledger {
                let keys: Vec<String> = to_send
                    .iter()
                    .map(|rel| ledger::key(&rel.anime_id, rel.episode))
                    .collect();
                if let Err(release_err) = ledger::release(env, &keys).await {
                    worker::console_error!("Failed to release ledger claims: {:?}", release_err);
                }
            }
            return Err(e);
        }
    };

    if managed_by_ledger {
        // Episodes that reached no channel keep no claim, so the next tick retries.
        let delivered_keys: Vec<String> = delivered
            .iter()
            .map(|rel| ledger::key(&rel.anime_id, rel.episode))
            .collect();
        let undelivered: Vec<String> = to_send
            .iter()
            .map(|rel| ledger::key(&rel.anime_id, rel.episode))
            .filter(|key| !delivered_keys.contains(key))
            .collect();
        // An empty commit is a wasted round trip and storage write.
        if !delivered_keys.is_empty() {
            if let Err(e) = ledger::commit(env, &delivered_keys).await {
                worker::console_error!("Failed to commit ledger entries: {:?}", e);
            }
        }
        if !undelivered.is_empty() {
            let _ = ledger::release(env, &undelivered).await;
        }
    }

    apply_delivered_episodes(tracked, &delivered);

    // Written unconditionally once a delivery happened: the snapshot may have been
    // reconciled forward above, and skipping that write is what let a stale copy
    // roll a committed episode back.
    if let Ok(json_str) = serde_json::to_string(tracked) {
        if let Ok(put) = kv.put("tracked_anime", json_str) {
            let _ = put.execute().await;
        }
    }

    Ok(delivered.len())
}

#[event(scheduled)]
pub async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let tick_started = profile::now_ms();
    let mut profile = TickProfile::default();

    let kv = match env.kv("ANIME_KV") {
        Ok(store) => store,
        Err(e) => {
            worker::console_error!("Failed to access ANIME_KV: {:?}", e);
            return;
        }
    };

    // The KV read and the upstream sweep are independent, so run them together:
    // the Samehadaku round trip is ~670 ms and would otherwise sit idle while a
    // ~100 ms KV read finishes first.
    let tracked_read = kv.get("tracked_anime");
    let tracked_future = async {
        let started = profile::now_ms();
        let result = tracked_read.json::<Vec<TrackedAnime>>().await;
        (result, profile::now_ms().saturating_sub(started).max(0))
    };
    let sources_future = collect_sources(&kv);
    let (sources, (tracked_result, kv_read_ms)) =
        futures_util::future::join(sources_future, tracked_future).await;
    profile.kv_read = kv_read_ms;
    profile.samehadaku = sources.samehadaku_ms;
    profile.telegram = sources.telegram_ms;

    let mut tracked = match tracked_result {
        Ok(Some(list)) => list,
        Ok(None) => {
            let default_list = default_tracked_list();
            if let Ok(json_str) = serde_json::to_string(&default_list) {
                if let Ok(put) = kv.put("tracked_anime", json_str) {
                    let _ = put.execute().await;
                }
            }
            default_list
        }
        Err(e) => {
            worker::console_error!("Error reading tracked_anime from KV: {:?}", e);
            default_tracked_list()
        }
    };

    merge_tracked_defaults(&mut tracked);

    // The Otakudesu sweep is deliberately NOT run here. otakudesu.blog answers
    // HTTP 403 to subrequests originating from a cron invocation while
    // accepting the identical request from an HTTP invocation, so a sweep from
    // this handler can only fail. Worse, it would consume the shared 15 minute
    // throttle window and starve the externally triggered run that can succeed.
    // The sweep lives in the `/trigger` route, driven by an external scheduler.
    let otakudesu_started = profile::now_ms();
    let candidates = sources.candidates;
    worker::console_log!("Otakudesu sweep delegated to the external /trigger scheduler");
    profile.record(Stage::Otakudesu, otakudesu_started, profile::now_ms());

    let matching_started = profile::now_ms();
    let releases = match_releases(&tracked, candidates);
    profile.record(Stage::Matching, matching_started, profile::now_ms());

    if !releases.is_empty() {
        worker::console_log!("Detected {} new releases!", releases.len());

        let webhook_url = match env.secret("DISCORD_WEBHOOK_URL") {
            Ok(sec) => sec.to_string(),
            Err(e) => {
                worker::console_error!("Missing secret DISCORD_WEBHOOK_URL: {:?}", e);
                profile.total = profile::now_ms().saturating_sub(tick_started);
                profile.log();
                return;
            }
        };

        // Claim -> send -> commit, so an episode another invocation already
        // announced is not announced again from this tick's stale snapshot.
        match deliver_unclaimed_releases(&env, &kv, &mut tracked, &releases, &webhook_url).await {
            Ok(delivered) if delivered < releases.len() => {
                worker::console_log!(
                    "{} of {} releases were delivered; the rest stay queued",
                    delivered,
                    releases.len()
                );
            }
            Ok(_) => {}
            Err(e) => {
                worker::console_error!("Failed to send Discord webhook: {:?}", e);
            }
        }
    }

    profile.total = profile::now_ms().saturating_sub(tick_started);
    profile.log();
}

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/health", |_, _| {
            Response::ok(
                r#"{"status":"healthy","service":"anime-pulse-worker","runtime":"rust-wasm"}"#,
            )
        })
        .post_async("/test-webhook", |req, ctx| async move {
            if let Err(denied) = auth::require(&req, &ctx.env) {
                return Ok(denied);
            }
            let secret_res = ctx.env.secret("DISCORD_WEBHOOK_URL");
            match secret_res {
                Ok(sec) => {
                    let url_str = sec.to_string();
                    let test_release = vec![EpisodeRelease {
                        anime_id: "ao-no-hako-s2".into(),
                        title: "Ao no Hako Season 2".into(),
                        episode: 3,
                        source: "Worker Diagnostic".into(),
                        url: "https://samehadaku.li".into(),
                        thumbnail_url: Some(
                            "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx170942-KKcLfQzV57nG.jpg".into(),
                        ),
                        image_url: Some(
                            "https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into(),
                        ),
                    }];
                    match discord::send_discord_webhook(&url_str, &test_release).await {
                        Ok(_) => Response::ok("Webhook diagnostic test sent successfully"),
                        Err(e) => {
                            worker::console_error!("Error sending webhook: {:?}", e);
                            Response::error("Failed to send webhook", 500)
                        }
                    }
                }
                Err(e) => {
                    worker::console_error!("Secret DISCORD_WEBHOOK_URL error: {:?}", e);
                    Response::error("Missing webhook configuration", 500)
                }
            }
        })
        .get_async("/schedule", |_, _| async move {
            match sokuja::fetch_schedule().await {
                Ok(rows) => {
                    let payload: Vec<serde_json::Value> = rows
                        .into_iter()
                        .map(|r| {
                            serde_json::json!({
                                "day": r.day,
                                "time": r.time,
                                "title": r.title,
                                "url": r.url
                            })
                        })
                        .collect();
                    match serde_json::to_string(&payload) {
                        Ok(body) => {
                            let mut resp = Response::ok(body)?;
                            resp.headers_mut().set("Content-Type", "application/json")?;
                            Ok(resp)
                        }
                        Err(e) => {
                            worker::console_error!("Schedule serialization error: {:?}", e);
                            Response::error("Internal server error", 500)
                        }
                    }
                }
                Err(e) => {
                    worker::console_error!("Sokuja schedule unavailable: {:?}", e);
                    Response::error("Schedule source unavailable", 502)
                }
            }
        })
        .get_async("/tracked", |_, ctx| async move {
            let kv = match ctx.env.kv("ANIME_KV") {
                Ok(store) => store,
                Err(e) => {
                    worker::console_error!("KV error: {:?}", e);
                    return Response::error("Failed to access storage", 500);
                }
            };
            let mut tracked = match kv.get("tracked_anime").json::<Vec<TrackedAnime>>().await {
                Ok(Some(list)) => list,
                Ok(None) => default_tracked_list(),
                Err(e) => {
                    worker::console_error!("Read error: {:?}", e);
                    default_tracked_list()
                }
            };
            merge_tracked_defaults(&mut tracked);

            match serde_json::to_string(&tracked) {
                Ok(json_str) => {
                    let mut resp = Response::ok(json_str)?;
                    resp.headers_mut().set("Content-Type", "application/json")?;
                    Ok(resp)
                }
                Err(e) => {
                    worker::console_error!("Serialization error: {:?}", e);
                    Response::error("Internal server error", 500)
                }
            }
        })
        .post_async("/ledger-selftest", |req, ctx| async move {
            if let Err(denied) = auth::require(&req, &ctx.env) {
                return Ok(denied);
            }
            // Claims one canary key twice through the real Durable Object. The
            // second claim must be refused, otherwise the dedupe that protects
            // against duplicate cards is not actually in force.
            //
            // The millisecond stamp goes in the id, never through a u32 cast: the
            // epoch in millis is ~1.7e12, so `as u32` saturates to 4294967295 and
            // every selftest would collide with the previous one inside the pending
            // TTL, reporting a false failure (review finding on #44).
            let canary_id = format!("__selftest-{}", Date::now().as_millis());
            let canary = vec![ledger::key(&canary_id, 0)];
            let first = match ledger::claim(&ctx.env, &canary).await {
                Ok(claim) => claim.granted.len(),
                Err(e) => {
                    worker::console_error!("Ledger selftest claim failed: {:?}", e);
                    return Response::error("Ledger unavailable", 503);
                }
            };
            let second = match ledger::claim(&ctx.env, &canary).await {
                Ok(claim) => claim.granted.len(),
                Err(e) => {
                    worker::console_error!("Ledger selftest re-claim failed: {:?}", e);
                    return Response::error("Ledger unavailable", 503);
                }
            };
            let _ = ledger::release(&ctx.env, &canary).await;
            let ok = first == 1 && second == 0;
            let mut resp = Response::ok(format!(
                r#"{{"ok":{ok},"first_claim":{first},"second_claim":{second}}}"#
            ))?;
            resp.headers_mut().set("Content-Type", "application/json")?;
            Ok(resp)
        })
        .post_async("/trigger", |req, ctx| async move {
            if let Err(denied) = auth::require(&req, &ctx.env) {
                return Ok(denied);
            }
            let tick_started = profile::now_ms();
            let mut profile = TickProfile::default();

            let kv = match ctx.env.kv("ANIME_KV") {
                Ok(store) => store,
                Err(e) => {
                    worker::console_error!("KV error: {:?}", e);
                    return Response::error("Failed to access storage", 500);
                }
            };
            let tracked_read = kv.get("tracked_anime");
            let tracked_future = async {
                let started = profile::now_ms();
                let result = tracked_read.json::<Vec<TrackedAnime>>().await;
                (result, profile::now_ms().saturating_sub(started).max(0))
            };
            let sources_future = collect_sources(&kv);
            let (sources, (tracked_result, kv_read_ms)) =
                futures_util::future::join(sources_future, tracked_future).await;
            profile.kv_read = kv_read_ms;
            profile.samehadaku = sources.samehadaku_ms;
            profile.telegram = sources.telegram_ms;

            let mut tracked = match tracked_result {
                Ok(Some(list)) => list,
                Ok(None) => default_tracked_list(),
                Err(e) => {
                    worker::console_error!("Error reading KV: {:?}", e);
                    return Response::error("Failed to read storage", 500);
                }
            };
            merge_tracked_defaults(&mut tracked);

            let otakudesu_started = profile::now_ms();
            let mut candidates = sources.candidates;
            if throttle_due(&kv, OTAKUDESU_LAST_CHECK_KEY, OTAKUDESU_INTERVAL_MS).await {
                mark_throttled(&kv, OTAKUDESU_LAST_CHECK_KEY).await;
                candidates.extend(sweep_otakudesu(&tracked).await);
            } else {
                worker::console_log!("Otakudesu sweep skipped, next window not reached");
            }
            profile.record(Stage::Otakudesu, otakudesu_started, profile::now_ms());

            let matching_started = profile::now_ms();
            let releases = match_releases(&tracked, candidates);
            profile.record(Stage::Matching, matching_started, profile::now_ms());

            let count = releases.len();
            if !releases.is_empty() {
                let webhook_url = match ctx.env.secret("DISCORD_WEBHOOK_URL") {
                    Ok(sec) => sec.to_string(),
                    Err(e) => {
                        worker::console_error!("Missing secret: {:?}", e);
                        return Response::error("Configuration error", 500);
                    }
                };
                match deliver_unclaimed_releases(
                    &ctx.env,
                    &kv,
                    &mut tracked,
                    &releases,
                    &webhook_url,
                )
                .await
                {
                    Ok(delivered) => {
                        if delivered < count {
                            worker::console_log!(
                                "{} of {} releases were delivered; the rest stay queued",
                                delivered,
                                count
                            );
                        }
                    }
                    Err(e) => {
                        worker::console_error!("Failed to send webhook: {:?}", e);
                        return Response::error("Failed to deliver notification", 502);
                    }
                }
            }
            profile.total = profile::now_ms().saturating_sub(tick_started);
            profile.log();
            let profile_json = serde_json::to_string(&profile).unwrap_or_else(|_| "{}".to_string());
            let mut resp = Response::ok(format!(
                r#"{{"triggered":true,"new_releases":{},"profile":{}}}"#,
                count, profile_json
            ))?;
            resp.headers_mut().set("Content-Type", "application/json")?;
            Ok(resp)
        })
        .run(req, env)
        .await
}

#[cfg(test)]
mod lib_tests {
    use super::parse_stamp;

    #[test]
    fn parses_quoted_and_bare_stamps() {
        // KvStore::put serialises a String through serde_json, so the stored
        // value reads back quoted. A bare form must also work.
        assert_eq!(parse_stamp("\"1730000000000\""), Some(1730000000000));
        assert_eq!(parse_stamp("1730000000000"), Some(1730000000000));
        assert_eq!(parse_stamp("  \"1730000000000\"  "), Some(1730000000000));
    }

    #[test]
    fn rejects_garbage_stamps() {
        assert_eq!(parse_stamp(""), None);
        assert_eq!(parse_stamp("not-a-number"), None);
        assert_eq!(parse_stamp("\"\""), None);
    }
}

#[cfg(test)]
mod match_tests {
    use super::*;

    fn tracked(id: &str, title: &str, last_episode: u32) -> TrackedAnime {
        TrackedAnime {
            id: id.to_string(),
            title: title.to_string(),
            last_episode,
            url: String::new(),
            thumbnail_url: None,
            image_url: None,
        }
    }

    fn candidate(title: &str, episode: u32, source: &'static str) -> ReleaseCandidate {
        ReleaseCandidate {
            title: title.to_string(),
            episode,
            url: format!("https://example.test/{episode}"),
            source,
            anime_id: None,
        }
    }

    #[test]
    fn advances_episode_and_emits_one_release() {
        let mut list = vec![tracked(
            "shiotaiou",
            "Shiotaiou no Satou-san ga Ore ni dake Amai",
            3,
        )];
        let out = match_releases(
            &list,
            vec![candidate(
                "Shiotaiou no Satou-san ga Ore ni dake Amai",
                4,
                "Samehadaku API",
            )],
        );

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].episode, 4);
        assert_eq!(out[0].source, "Samehadaku API");
        // Detection alone must not advance state: the caller commits only after
        // the webhook confirms delivery (issue #31).
        assert_eq!(list[0].last_episode, 3);
        assert_eq!(apply_delivered_episodes(&mut list, &out), 1);
        assert_eq!(list[0].last_episode, 4);
    }

    #[test]
    fn same_episode_from_two_sources_yields_a_single_notification() {
        let mut list = vec![tracked("kyouran", "Kyouran Reijou Nia Liston", 2)];
        let out = match_releases(
            &list,
            vec![
                candidate("Kyouran Reijou Nia Liston", 3, "Samehadaku API"),
                candidate(
                    "Kyouran Reijou Nia Liston: Byoujaku Reijou ni Tensei",
                    3,
                    "Sokuja (Telegram)",
                ),
            ],
        );

        assert_eq!(out.len(), 1, "duplicate episode must not notify twice");
        // Samehadaku is supplied first, so its link wins.
        assert_eq!(out[0].source, "Samehadaku API");
        apply_delivered_episodes(&mut list, &out);
        assert_eq!(list[0].last_episode, 3);
    }

    #[test]
    fn older_or_equal_episodes_are_ignored() {
        let list = vec![tracked("psyren", "Psyren", 5)];
        let out = match_releases(
            &list,
            vec![
                candidate("Psyren", 5, "Samehadaku API"),
                candidate("Psyren", 4, "Samehadaku API"),
            ],
        );

        assert!(out.is_empty());
        assert_eq!(list[0].last_episode, 5);
    }

    #[test]
    fn id_bound_candidate_matches_even_when_titles_differ() {
        // Otakudesu candidates carry the tracked id because the series URL came
        // from that entry; title similarity must not be required.
        let mut list = vec![tracked(
            "tensei-shita-daiseijo",
            "Tensei shita Daiseijo wa, Seijo de Aru Koto wo Hitakakusu",
            3,
        )];
        let out = match_releases(
            &list,
            vec![ReleaseCandidate {
                title: String::new(),
                episode: 4,
                url: "https://otakudesu.blog/episode/tsds-episode-4-sub-indo/".to_string(),
                source: "Otakudesu",
                anime_id: Some("tensei-shita-daiseijo".to_string()),
            }],
        );

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].episode, 4);
        assert_eq!(out[0].source, "Otakudesu");
        apply_delivered_episodes(&mut list, &out);
        assert_eq!(list[0].last_episode, 4);
    }

    #[test]
    fn id_bound_candidate_ignores_other_entries() {
        let mut list = vec![
            tracked("a", "Kyouran Reijou Nia Liston", 2),
            tracked("b", "Psyren", 1),
        ];
        let out = match_releases(
            &list,
            vec![ReleaseCandidate {
                title: "Psyren".to_string(),
                episode: 5,
                url: "https://otakudesu.blog/episode/psyren-episode-5-sub-indo/".to_string(),
                source: "Otakudesu",
                anime_id: Some("a".to_string()),
            }],
        );

        // Bound to "a" by id, so it must not touch "b" despite the matching title.
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].anime_id, "a");
        apply_delivered_episodes(&mut list, &out);
        assert_eq!(list[0].last_episode, 5);
        assert_eq!(list[1].last_episode, 1);
    }

    #[test]
    fn unrelated_titles_never_match() {
        let list = vec![tracked("psyren", "Psyren", 1)];
        let out = match_releases(
            &list,
            vec![candidate(
                "Dragon Ball Super: Beerus",
                2,
                "Sokuja (Telegram)",
            )],
        );

        assert!(out.is_empty());
        assert_eq!(list[0].last_episode, 1);
    }

    /// Regression for issue #31. `Ao no Hako Season 2` received a card for Ep 3
    /// **and** Ep 4 in one sweep because dedupe relied on `last_episode`
    /// advancing on the first match, which only holds while candidates arrive in
    /// ascending order.
    #[test]
    fn one_title_yields_one_release_even_when_sources_disagree_on_order() {
        let mut list = vec![tracked("ao-no-hako-s2", "Ao no Hako Season 2", 2)];

        // Newest first: no ascending order to lean on.
        let out = match_releases(
            &list,
            vec![
                candidate("Ao no Hako Season 2", 4, "Sokuja (Telegram)"),
                candidate("Ao no Hako 2nd Season", 3, "Sokuja (Telegram)"),
                candidate("Ao no Hako Season 2", 3, "Samehadaku API"),
            ],
        );

        assert_eq!(out.len(), 1, "one release must produce exactly one card");
        assert_eq!(out[0].episode, 4, "the highest episode must win");

        apply_delivered_episodes(&mut list, &out);
        assert_eq!(list[0].last_episode, 4);
    }

    /// Regression for issue #31, second half: a release that reaches no webhook
    /// target must leave `last_episode` untouched so the next tick retries it.
    #[test]
    fn undelivered_release_does_not_advance_state() {
        let mut list = vec![tracked("psyren", "Psyren", 2)];
        let detected = match_releases(&list, vec![candidate("Psyren", 3, "Samehadaku API")]);
        assert_eq!(detected.len(), 1);

        // Discord reported zero delivered targets.
        let delivered: Vec<EpisodeRelease> = Vec::new();
        assert_eq!(apply_delivered_episodes(&mut list, &delivered), 0);
        assert_eq!(
            list[0].last_episode, 2,
            "a failed delivery must stay queued for the next tick"
        );

        // Once a target accepts it, the state moves exactly once.
        assert_eq!(apply_delivered_episodes(&mut list, &detected), 1);
        assert_eq!(list[0].last_episode, 3);
        assert_eq!(
            apply_delivered_episodes(&mut list, &detected),
            0,
            "replaying the same delivery must be idempotent"
        );
    }

    /// Review finding on #44: a tick that delivers one title must not write its
    /// stale copy of another title back over an episode somebody else already
    /// committed. Without reconciliation the episode would be re-checked against
    /// the ledger on every tick for the whole retention window, and the watchlist
    /// in KV would never catch up.
    #[test]
    fn a_deduped_episode_is_reconciled_before_the_watchlist_is_written() {
        let mut list = vec![
            tracked("psyren", "Psyren", 3),
            tracked("ao-no-hako-s2", "Ao no Hako Season 2", 2),
        ];

        // Another invocation delivered Psyren ep 4; this tick's snapshot predates it.
        let refused = vec![ledger::key("psyren", 4)];
        assert_eq!(reconcile_already_delivered(&mut list, &refused), 1);
        assert_eq!(list[0].last_episode, 4, "the snapshot catches up");

        // This tick delivers a different title, then writes the whole list back.
        let other = EpisodeRelease {
            anime_id: "ao-no-hako-s2".to_string(),
            title: "Ao no Hako Season 2".to_string(),
            episode: 3,
            source: "Samehadaku API".to_string(),
            url: "https://example.test/ep".to_string(),
            thumbnail_url: None,
            image_url: None,
        };
        apply_delivered_episodes(&mut list, std::slice::from_ref(&other));

        assert_eq!(
            list[0].last_episode, 4,
            "the committed episode survives the write instead of rolling back to 3"
        );
        assert_eq!(list[1].last_episode, 3);
    }

    #[test]
    fn selftest_canary_ids_are_unique_per_millisecond() {
        // Mirrors the endpoint: the timestamp belongs in the id, because a u32 cast
        // of epoch millis saturates and freezes the canary.
        let a = ledger::key(&format!("__selftest-{}", 1_792_847_000_001u64), 0);
        let b = ledger::key(&format!("__selftest-{}", 1_792_847_000_002u64), 0);
        assert_ne!(a, b, "two selftests must not share a canary key");
        assert_eq!(
            ledger::parse_key(&a),
            Some(("__selftest-1792847000001".to_string(), 0)),
            "the id survives the round trip, so the ledger can reconcile it"
        );
        // The cast the endpoint avoids: `Date::now().as_millis()` is f64, and a
        // float-to-int cast saturates rather than truncating, so every run collapses
        // onto the same canary. An integer cast would have truncated instead, which
        // is how this was missed.
        let millis = 1_792_847_000_001f64;
        assert_eq!(millis as u32, u32::MAX, "f64 to u32 saturates");
        assert_eq!(
            1_792_847_000_001u64 as u32, 1_845_637_569u32,
            "u64 to u32 truncates: a different failure mode, not the one to guard against"
        );
    }

    #[test]
    fn reconciling_never_moves_a_watchlist_entry_backwards() {
        let mut list = vec![tracked("psyren", "Psyren", 9)];
        // A ledger record for an older episode must not undo later progress.
        assert_eq!(
            reconcile_already_delivered(&mut list, &[ledger::key("psyren", 4)]),
            0
        );
        assert_eq!(list[0].last_episode, 9);
    }

    /// Two invocations that read the watchlist before either commits will both
    /// match the same episode. Nothing in the delivery path prevents it: the only
    /// guard is `rel.episode > item.last_episode`, evaluated against the snapshot
    /// each invocation read at the start of its own tick.
    ///
    /// This is the duplicate-card hazard. It is not reachable by two ticks racing
    /// inside one isolate; it is reachable because the cron path and the external
    /// /trigger path run 25 seconds apart while a KV write can take up to 60
    /// seconds to be visible, and because any retry or manual /trigger widens it.
    #[test]
    fn concurrent_ticks_from_one_snapshot_both_deliver_the_same_episode() {
        let snapshot = default_tracked_list();
        let target = snapshot[0].clone();
        let next = target.last_episode + 1;
        let make_candidate = || ReleaseCandidate {
            title: target.title.clone(),
            episode: next,
            url: "https://example.test/ep".to_string(),
            source: "Samehadaku API",
            anime_id: Some(target.id.clone()),
        };

        // Both ticks read `snapshot` before either writes anything back.
        let mut tick_a = snapshot.clone();
        let mut tick_b = snapshot.clone();

        let matched_a = match_releases(&tick_a, vec![make_candidate()]);
        let matched_b = match_releases(&tick_b, vec![make_candidate()]);
        assert_eq!(matched_a.len(), 1, "first tick matches the release");
        assert_eq!(
            matched_b.len(),
            1,
            "second tick matches the same release from the same stale pre-state"
        );

        // Both deliver, both commit: Discord has received two identical cards.
        assert_eq!(apply_delivered_episodes(&mut tick_a, &matched_a), 1);
        assert_eq!(apply_delivered_episodes(&mut tick_b, &matched_b), 1);

        // State converges afterwards, which is why the bug never shows up as
        // corruption - only as a duplicate notification nobody can explain.
        assert_eq!(tick_a[0].last_episode, tick_b[0].last_episode);
    }

    /// Regression for issue #33: a title added to the code defaults must appear
    /// even when KV is missing it, and must land at the episode that was out
    /// when it joined, not at zero.
    #[test]
    fn default_list_carries_hotaru_no_yomeiri_at_episode_three() {
        let defaults = default_tracked_list();
        let entry = defaults
            .iter()
            .find(|t| t.id == "hotaru-no-yomeiri")
            .expect("Hotaru no Yomeiri must be in the default watchlist");
        assert_eq!(entry.title, "Hotaru no Yomeiri");
        assert_eq!(
            entry.last_episode, 3,
            "tracking starts at Ep 4, so the three episodes already out must not replay"
        );
        assert!(entry.url.contains("hotaru-yomeiri-sub-indo"));
        assert!(entry.thumbnail_url.is_some());
        assert!(entry.image_url.is_some());
    }

    /// Regression for issue #33: a KV record written before the title existed
    /// must gain it on the next tick, because `merge_tracked_defaults` runs
    /// every tick rather than only when KV is empty.
    #[test]
    fn merge_adds_missing_default_without_disturbing_existing_state() {
        let mut list = vec![tracked("psyren", "Psyren", 7)];
        merge_tracked_defaults(&mut list);

        assert_eq!(
            list[0].last_episode, 7,
            "merging defaults must never reset a live counter"
        );
        let hotaru = list
            .iter()
            .find(|t| t.id == "hotaru-no-yomeiri")
            .expect("merge must append the missing default");
        assert_eq!(hotaru.last_episode, 3);

        // Idempotent: a second merge must not duplicate the entry.
        let before = list.len();
        merge_tracked_defaults(&mut list);
        assert_eq!(list.len(), before, "merge must be idempotent");
    }

    /// Regression for issue #33: the live Sokuja/Samehadaku post title must
    /// resolve to the new entry, otherwise the watchlist addition is inert.
    #[test]
    fn hotaru_no_yomeiri_release_title_matches_its_entry() {
        let list = vec![tracked("hotaru-no-yomeiri", "Hotaru no Yomeiri", 3)];
        let out = match_releases(
            &list,
            vec![candidate(
                "Hotaru no Yomeiri Episode 4 Subtitle Indonesia",
                4,
                "Sokuja (Telegram)",
            )],
        );
        assert_eq!(out.len(), 1, "episode 4 must be detected");
        assert_eq!(out[0].anime_id, "hotaru-no-yomeiri");
        assert_eq!(out[0].episode, 4);

        // Episode 3 is already the baseline, so it must stay quiet.
        let out = match_releases(
            &list,
            vec![candidate(
                "Hotaru no Yomeiri Episode 3 Subtitle Indonesia",
                3,
                "Sokuja (Telegram)",
            )],
        );
        assert!(
            out.is_empty(),
            "the baseline episode must not be re-announced"
        );
    }

    /// Review follow-up on #35: per-series body sizes must not fill every log
    /// slot with what is really one underlying cause.
    #[test]
    fn failure_reasons_group_by_cause_not_by_size() {
        let a = "otakudesu series page exposed no episode links (31078 byte body)";
        let b = "otakudesu series page exposed no episode links (31102 byte body)";
        assert_eq!(group_failure_reason(a), group_failure_reason(b));
        assert_eq!(
            group_failure_reason(a),
            "otakudesu series page exposed no episode links (# byte body)"
        );
    }

    /// A status code is a distinct cause and must survive grouping; only size
    /// figures are noise.
    #[test]
    fn failure_reasons_keep_http_status_codes_distinct() {
        assert_ne!(
            group_failure_reason("otakudesu series returned HTTP 403"),
            group_failure_reason("otakudesu series returned HTTP 500")
        );
        assert_eq!(
            group_failure_reason("otakudesu series returned HTTP 403"),
            "otakudesu series returned HTTP 403"
        );
    }

    /// Undecoded-compression messages carry both an encoding name and a size;
    /// different encodings must not collapse into one slot.
    #[test]
    fn failure_reasons_keep_compression_kind_distinct() {
        let zstd = "otakudesu body still zstd compressed (9601 bytes): runtime did not decode";
        let gzip = "otakudesu body still gzip compressed (9136 bytes): runtime did not decode";
        assert_ne!(group_failure_reason(zstd), group_failure_reason(gzip));
        assert!(group_failure_reason(zstd).contains("zstd"));
        assert!(group_failure_reason(gzip).contains("gzip"));
    }
}
