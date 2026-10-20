pub mod discord;
pub mod matcher;
pub mod models;
pub mod sokuja;
pub mod telegram;

use models::{EpisodeRelease, SamehadakuPost, TrackedAnime};
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
struct ReleaseCandidate {
    title: String,
    episode: u32,
    url: String,
    source: &'static str,
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

/// True when the Telegram bridge is due for another poll.
///
/// Fails open: if the stamp is missing or unreadable we poll, because a stray
/// extra fetch costs far less than a silently dead source.
async fn telegram_due(kv: &KvStore) -> bool {
    let now = chrono::Utc::now().timestamp_millis();
    match kv.get(TELEGRAM_LAST_CHECK_KEY).text().await {
        Ok(Some(raw)) => match parse_stamp(&raw) {
            Some(last) => now.saturating_sub(last) >= TELEGRAM_INTERVAL_MS,
            None => true,
        },
        _ => true,
    }
}

/// Parses the throttle stamp written by [`mark_telegram_checked`].
///
/// `KvStore::put` serialises a `String` through `serde_json`, so the stored
/// value comes back as `"<millis>"` with the quotes intact. Accepting both the
/// quoted and the bare form keeps the throttle from silently failing open on
/// every tick, which would poll the channel every 60 s.
fn parse_stamp(raw: &str) -> Option<i64> {
    raw.trim().trim_matches('"').trim().parse::<i64>().ok()
}

/// Records the current time as the last Telegram poll attempt.
///
/// Marked on attempt rather than success so an outage cannot escalate into a
/// poll every 60 s plus a 590 KB fallback download each time.
async fn mark_telegram_checked(kv: &KvStore) {
    let stamp = chrono::Utc::now().timestamp_millis().to_string();
    if let Ok(put) = kv.put(TELEGRAM_LAST_CHECK_KEY, stamp) {
        if let Err(e) = put.execute().await {
            worker::console_error!("Could not persist Telegram throttle stamp: {:?}", e);
        }
    }
}

/// Collects release candidates from every configured source and matches them
/// against the tracked watchlist.
///
/// `kv` is used only for the Telegram bridge throttle stamp.
pub async fn check_for_releases(
    tracked: &mut [TrackedAnime],
    kv: &KvStore,
) -> Result<Vec<EpisodeRelease>> {
    let mut candidates: Vec<ReleaseCandidate> = Vec::new();

    match fetch_samehadaku_candidates().await {
        Ok(mut c) => candidates.append(&mut c),
        Err(e) => worker::console_error!("Source Samehadaku unavailable, skipping: {:?}", e),
    }

    if telegram_due(kv).await {
        match telegram::fetch_posts().await {
            Ok(posts) => {
                mark_telegram_checked(kv).await;
                worker::console_log!("Telegram bridge reported {} release posts", posts.len());
                for p in posts {
                    candidates.push(ReleaseCandidate {
                        title: p.title,
                        episode: p.episode,
                        url: p.url,
                        source: "Sokuja (Telegram)",
                    });
                }
            }
            Err(e) => {
                mark_telegram_checked(kv).await;
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
                            });
                        }
                    }
                    Err(e2) => {
                        worker::console_error!("Sokuja HTML fallback unavailable too: {:?}", e2)
                    }
                }
            }
        }
    } else {
        worker::console_log!("Telegram bridge skipped, next poll window not reached");
    }

    let mut detected_releases = Vec::new();

    for cand in candidates {
        for item in tracked.iter_mut() {
            if cand.episode > item.last_episode
                && matcher::matches_tracked(&item.title, &cand.title)
            {
                item.last_episode = cand.episode;
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
        }
    }

    Ok(detected_releases)
}

#[event(scheduled)]
pub async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let kv = match env.kv("ANIME_KV") {
        Ok(store) => store,
        Err(e) => {
            worker::console_error!("Failed to access ANIME_KV: {:?}", e);
            return;
        }
    };

    let mut tracked = match kv.get("tracked_anime").json::<Vec<TrackedAnime>>().await {
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

    let releases = match check_for_releases(&mut tracked, &kv).await {
        Ok(r) => r,
        Err(e) => {
            worker::console_error!("Error checking releases: {:?}", e);
            return;
        }
    };

    if !releases.is_empty() {
        worker::console_log!("Detected {} new releases!", releases.len());

        let webhook_url = match env.secret("DISCORD_WEBHOOK_URL") {
            Ok(sec) => sec.to_string(),
            Err(e) => {
                worker::console_error!("Missing secret DISCORD_WEBHOOK_URL: {:?}", e);
                return;
            }
        };

        // Only commit updated episode state to KV if Discord webhook succeeds
        match discord::send_discord_webhook(&webhook_url, &releases).await {
            Ok(_) => {
                if let Ok(json_str) = serde_json::to_string(&tracked) {
                    if let Ok(put) = kv.put("tracked_anime", json_str) {
                        let _ = put.execute().await;
                    }
                }
            }
            Err(e) => {
                worker::console_error!("Failed to send Discord webhook: {:?}", e);
            }
        }
    }
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
        .post_async("/test-webhook", |_, ctx| async move {
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
        .post_async("/trigger", |_, ctx| async move {
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
                    worker::console_error!("Error reading KV: {:?}", e);
                    return Response::error("Failed to read storage", 500);
                }
            };
            merge_tracked_defaults(&mut tracked);

            let releases = match check_for_releases(&mut tracked, &kv).await {
                Ok(r) => r,
                Err(e) => {
                    worker::console_error!("Check error: {:?}", e);
                    return Response::error("Failed to check releases", 500);
                }
            };

            let count = releases.len();
            if !releases.is_empty() {
                let webhook_url = match ctx.env.secret("DISCORD_WEBHOOK_URL") {
                    Ok(sec) => sec.to_string(),
                    Err(e) => {
                        worker::console_error!("Missing secret: {:?}", e);
                        return Response::error("Configuration error", 500);
                    }
                };
                match discord::send_discord_webhook(&webhook_url, &releases).await {
                    Ok(_) => {
                        if let Ok(json_str) = serde_json::to_string(&tracked) {
                            if let Ok(put) = kv.put("tracked_anime", json_str) {
                                let _ = put.execute().await;
                            }
                        }
                    }
                    Err(e) => {
                        worker::console_error!("Failed to send webhook: {:?}", e);
                        return Response::error("Failed to deliver notification", 502);
                    }
                }
            }
            let mut resp = Response::ok(format!(r#"{{"triggered":true,"new_releases":{}}}"#, count))?;
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
