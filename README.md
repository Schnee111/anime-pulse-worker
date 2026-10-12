# anime-pulse-worker

Ultra-low-latency, zero-VPS-memory Cloudflare Worker written in Rust (`workers-rs` / WebAssembly) that tracks Indonesian anime releases and dispatches real-time native Discord Rich Embed notifications.

## Overview

Traditional cron-based aggregators poll streaming sites every 1–2 hours from a Linux host, wasting server memory and introducing up to 120 minutes of notification lag. `anime-pulse-worker` shifts tracking entirely to Cloudflare Edge:

- **1-Minute Precision:** Executes on a Cloudflare Cron Trigger (`* * * * *`).
- **Zero VPS Resource Usage:** Runs serverless on Cloudflare's global edge network (uses only ~1.4% of Cloudflare's free 100k daily request quota).
- **Lightweight Probes:** Queries Samehadaku's WordPress REST API projection (`?per_page=15&_fields=id,date,title,link`), consuming <2 KB wire data per minute.
- **Atomic State in KV:** Tracks watchlist items and episode progression via Cloudflare KV (`ANIME_KV`).
- **Clean Discord Embeds:** Directly dispatches Spectro Mint (`#6ee7b7`) embeds to Discord Webhooks with zero bot overhead.

## Architecture

```
Cloudflare Cron (* * * * *)
          │
          ▼
anime-pulse-worker (Rust Wasm)
   ├── Read Watchlist (ANIME_KV)
   ├── GET samehadaku.li/wp-json/wp/v2/posts (Gzip, 153B - 2KB)
   ├── Parse & Match Episode (regex + word token overlap)
   │
   └── [New Release Detected]
         ├── Update KV (last_episode)
         └── POST Discord Webhook (#anime-update)
```

## API Endpoints

- `GET /health` — Health check endpoint returning worker runtime status.
- `GET /tracked` — Returns the active watchlist and recorded episode states from KV.
- `POST /trigger` — Manually triggers a release scan and dispatches any detected updates.

## Configuration & Secrets

Set secrets via Wrangler CLI:

```bash
wrangler secret put DISCORD_WEBHOOK_URL
```

KV Namespace configuration in `wrangler.toml`:

```toml
[[kv_namespaces]]
binding = "ANIME_KV"
id = "anime_pulse_kv"
```

## Local Development & Testing

Prerequisites: Rust 1.80+ with `wasm32-unknown-unknown` target.

```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Run host unit tests
cargo test

# Check Wasm compilation
cargo check --target wasm32-unknown-unknown

# Lint with Clippy
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## License

MIT © Muhammad Daffa Ma'arif (Schnee)
