pub mod discord;
pub mod matcher;
pub mod models;

use models::{EpisodeRelease, SamehadakuPost, TrackedAnime};
use worker::*;

pub fn default_tracked_list() -> Vec<TrackedAnime> {
    vec![
        TrackedAnime {
            id: "koori-jouheki-s2".into(),
            title: "Koori no Jouheki Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/koori-jouheki-s2-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/a7ah5by0Ffw/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "ao-no-hako-s2".into(),
            title: "Ao no Hako Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/ao-hako-s2-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "yowaki-max-reijou".into(),
            title: "Yowaki Max Reijou nanoni, Ratsuwan Konyakusha-sama no Kake ni Notte Shimatta".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/yowaki-max-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/e41RGxVwJRs/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tensei-shita-daiseijo".into(),
            title: "Tensei shita Daiseijo wa, Seijo de Aru Koto wo Hitakakusu".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/tensei-shita-daiseijo-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/9oBywQ403ZA/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "futsutsuka-na-akujo".into(),
            title: "Futsutsuka na Akujo dewa Gozaimasu ga".into(),
            last_episode: 11,
            url: "https://otakudesu.blog/anime/futsutsuka-akujo-gozaimasu-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/nR_vqhD2rGM/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tempal-chikara".into(),
            title: "Tempal: Item no Chikara".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/tempal-chikara-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/e1Km0FqUZl0/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "fx-senshi-kurumi".into(),
            title: "FX Senshi Kurumi-chan".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/fx-senshi-kurumi-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/7rxIZ3z0S4s/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "saikyosoubi".into(),
            title: "Mezametara Saikyou Soubi to Uchuusenmochi Datta node, Ikkodate Mezashite Youhei toshite Jiyuu ni Ikitai".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/saikyosoubi-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/Z9ofT53sz2U/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "yasei-last-boss-s2".into(),
            title: "Yasei no Last Boss ga Arawareta! Season 2".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/yasei-last-bos-s2-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/KVT_ODZn9Ys/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "romelia-senki".into(),
            title: "Romelia Senki".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/romelia-senki-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/PxXPcoo8uGM/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "kyouran-reijou-nia-liston".into(),
            title: "Kyouran Reijou Nia Liston".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/kyouran-reijou-liston-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/gxG4vntLtbk/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "doumo-suki-na-majo".into(),
            title: "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu.".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/doumo-suki-majo-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/hGRtocAh3iw/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "magical-explorer".into(),
            title: "Magical★Explorer".into(),
            last_episode: 3,
            url: "https://otakudesu.blog/anime/magic-explor-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/XqrBfyUNYZs/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "seitokai-ni-mo-ana-wa-aru".into(),
            title: "Seitokai ni mo Ana wa Aru!".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/seitokai-ana-aru-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/VTRVLnzhl90/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "zombie-sagashitemasu".into(),
            title: "#Zombie Sagashitemasu".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/zombie-saga-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/_0kVb8-uRSI/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "sekai-saikyou-no-majo".into(),
            title: "Sekai Saikyou no Majo, Hajimemashita".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/sekai-hajimemashita-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/snJZD9vxfHY/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "chitose-kun-ramune-p2".into(),
            title: "Chitose-kun wa Ramune Bin no Naka Part 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/chitose-ramune-bin-naka-p2-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/wC7FYPFHLeQ/maxresdefault.jpg".into()),
        },
        TrackedAnime {
            id: "tantei-wa-mou-shindeiru-s2".into(),
            title: "Tantei wa Mou, Shindeiru. Season 2".into(),
            last_episode: 2,
            url: "https://otakudesu.blog/anime/tante-mou-shindeiru-s2-sub-indo/".into(),
            image_url: Some("https://img.youtube.com/vi/MghFONe--0Q/maxresdefault.jpg".into()),
        },
    ]
}

/// Core probe logic: queries Samehadaku REST API and detects new episodes
pub async fn check_for_releases(tracked: &mut [TrackedAnime]) -> Result<Vec<EpisodeRelease>> {
    let headers = Headers::new();
    headers.set(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)",
    )?;
    headers.set("Accept-Encoding", "gzip, deflate")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);

    let req = Request::new_with_init(
        "https://samehadaku.li/wp-json/wp/v2/posts?per_page=15&_fields=id,date,title,link",
        &init,
    )?;

    let mut resp = Fetch::Request(req).send().await?;
    if resp.status_code() != 200 {
        worker::console_warn!(
            "Samehadaku API responded with status {}",
            resp.status_code()
        );
        return Ok(Vec::new());
    }

    let posts: Vec<SamehadakuPost> = resp.json().await?;
    let mut detected_releases = Vec::new();

    for post in posts {
        let raw_title = &post.title.rendered;
        if let Some((parsed_title, episode_num)) = matcher::parse_release_title(raw_title) {
            for item in tracked.iter_mut() {
                if matcher::matches_tracked(&item.title, &parsed_title)
                    && episode_num > item.last_episode
                {
                    item.last_episode = episode_num;
                    detected_releases.push(EpisodeRelease {
                        anime_id: item.id.clone(),
                        title: item.title.clone(),
                        episode: episode_num,
                        source: "Samehadaku API".into(),
                        url: post.link.clone(),
                        image_url: item.image_url.clone(),
                    });
                }
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

    // Load tracked anime from KV or initialize defaults
    let mut tracked: Vec<TrackedAnime> = match kv.get("tracked_anime").json().await {
        Ok(Some(list)) => list,
        _ => {
            let defaults = default_tracked_list();
            if let Ok(json_str) = serde_json::to_string(&defaults) {
                let _ = kv.put("tracked_anime", json_str).map(|p| p.execute());
            }
            defaults
        }
    };

    match check_for_releases(&mut tracked).await {
        Ok(releases) => {
            if !releases.is_empty() {
                worker::console_log!("Detected {} new releases!", releases.len());

                // Save updated tracked anime to KV
                if let Ok(json_str) = serde_json::to_string(&tracked) {
                    if let Ok(put) = kv.put("tracked_anime", json_str) {
                        let _ = put.execute().await;
                    }
                }

                // Dispatch to Discord Webhook
                if let Ok(webhook_url) = env.secret("DISCORD_WEBHOOK_URL") {
                    let url_str = webhook_url.to_string();
                    if let Err(e) = discord::send_discord_webhook(&url_str, &releases).await {
                        worker::console_error!("Failed to send Discord webhook: {:?}", e);
                    }
                } else {
                    worker::console_warn!("DISCORD_WEBHOOK_URL secret not configured");
                }
            }
        }
        Err(e) => {
            worker::console_error!("Error during scheduled release check: {:?}", e);
        }
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
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
                        image_url: Some(
                            "https://img.youtube.com/vi/uvvYMBLY-w4/maxresdefault.jpg".into(),
                        ),
                    }];
                    match discord::send_discord_webhook(&url_str, &test_release).await {
                        Ok(_) => Response::ok(format!(
                            "Webhook sent successfully to: {}...",
                            &url_str[..40]
                        )),
                        Err(e) => Response::error(format!("Error sending webhook: {e:?}"), 500),
                    }
                }
                Err(e) => Response::error(format!("Secret DISCORD_WEBHOOK_URL error: {e:?}"), 500),
            }
        })
        .get_async("/tracked", |_, ctx| async move {
            let kv = ctx.env.kv("ANIME_KV")?;
            let tracked: Vec<TrackedAnime> = kv
                .get("tracked_anime")
                .json()
                .await?
                .unwrap_or_else(default_tracked_list);
            Response::from_json(&tracked)
        })
        .post_async("/trigger", |_, ctx| async move {
            let kv = ctx.env.kv("ANIME_KV")?;
            let mut tracked: Vec<TrackedAnime> = kv
                .get("tracked_anime")
                .json()
                .await?
                .unwrap_or_else(default_tracked_list);

            let releases = check_for_releases(&mut tracked).await?;
            if !releases.is_empty() {
                if let Ok(json_str) = serde_json::to_string(&tracked) {
                    let _ = kv.put("tracked_anime", json_str)?.execute().await;
                }
                if let Ok(webhook_url) = ctx.env.secret("DISCORD_WEBHOOK_URL") {
                    let _ =
                        discord::send_discord_webhook(&webhook_url.to_string(), &releases).await;
                }
            }

            Response::from_json(&releases)
        })
        .run(req, env)
        .await
}
