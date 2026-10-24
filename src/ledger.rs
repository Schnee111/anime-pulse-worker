//! Strongly consistent delivery ledger.
//!
//! The watchlist lives in KV, which is eventually consistent: a write made by one
//! invocation can stay invisible to another for up to a minute. The delivery path
//! reads the watchlist at the start of a tick and only commits after the webhook
//! confirms, so two invocations that read the same pre-state both match the same
//! episode and both post a card for it. That is issue #43. The cron path fires
//! every minute and the external `/trigger` fires 25 seconds later, which is well
//! inside the propagation window, so the duplicate does not need the two ticks to
//! overlap - the second only needs a stale read.
//!
//! A KV-based guard cannot fix this, because the guard would be subject to the
//! same delay it is defending against. Instead one Durable Object instance (single
//! threaded, strongly consistent) owns the record of what has already been
//! announced.
//!
//! Lifecycle of an entry:
//!
//! ```text
//! claim   -> Pending (30 min)   blocks a second invocation, but self-heals if the
//!                               worker dies before it can send
//! commit  -> Final (120 days)   set only after the webhook confirmed delivery
//! release -> removed            set when the webhook failed, so the episode
//!                               stays queued for the next tick
//! ```
//!
//! Delivery-then-commit still governs `last_episode` in KV: nothing there advances
//! until Discord confirms. The ledger only answers "has this episode already been
//! claimed by somebody".

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::*;

/// How long a claim holds the episode before it is assumed abandoned.
///
/// Long enough to cover a slow webhook round trip on any tick, short enough that a
/// worker which dies between claim and commit cannot suppress an episode for long.
pub const PENDING_TTL_MS: i64 = 30 * 60 * 1000;

/// How long a delivered episode is remembered.
///
/// Only needs to outlive the window in which a stale watchlist read is possible,
/// but keeping a few months of history costs nothing (22 titles) and makes the
/// ledger useful when investigating a report of a duplicate or a miss.
pub const RETENTION_MS: i64 = 120 * 24 * 60 * 60 * 1000;

/// The identity of one announcement: this anime, this episode.
pub fn key(anime_id: &str, episode: u32) -> String {
    format!("{anime_id}:{episode}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryState {
    /// Claimed by an invocation that has not confirmed delivery yet.
    Pending,
    /// Delivered and confirmed. Blocks every future claim.
    Final,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub state: EntryState,
    /// When the entry entered its current state, epoch millis.
    pub at_ms: i64,
}

/// The stored decision state. Pure data with no runtime dependency, so the
/// semantics can be tested without a Durable Object.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ledger {
    pub entries: BTreeMap<String, Entry>,
}

impl Ledger {
    /// Returns the subset of `keys` this caller now owns.
    ///
    /// A key is claimable when it is unknown, or when its pending claim has aged
    /// past `PENDING_TTL_MS`. Finals are never claimable again.
    pub fn claim(&mut self, keys: &[String], now_ms: i64) -> Vec<String> {
        let mut granted = Vec::new();
        for key in keys {
            let claimable = match self.entries.get(key) {
                None => true,
                Some(entry) => match entry.state {
                    EntryState::Final => false,
                    EntryState::Pending => now_ms.saturating_sub(entry.at_ms) >= PENDING_TTL_MS,
                },
            };
            if !claimable {
                continue;
            }
            self.entries.insert(
                key.clone(),
                Entry {
                    state: EntryState::Pending,
                    at_ms: now_ms,
                },
            );
            granted.push(key.clone());
        }
        granted
    }

    /// Marks claimed keys as delivered. Called only after the webhook confirmed.
    pub fn commit(&mut self, keys: &[String], now_ms: i64) {
        for key in keys {
            self.entries.insert(
                key.clone(),
                Entry {
                    state: EntryState::Final,
                    at_ms: now_ms,
                },
            );
        }
    }

    /// Drops claims so the next tick can retry them.
    pub fn release(&mut self, keys: &[String]) {
        for key in keys {
            self.entries.remove(key);
        }
    }

    /// Removes finals past retention and pendings past their TTL.
    pub fn prune(&mut self, now_ms: i64) {
        self.entries.retain(|_, entry| match entry.state {
            EntryState::Final => now_ms.saturating_sub(entry.at_ms) < RETENTION_MS,
            EntryState::Pending => now_ms.saturating_sub(entry.at_ms) < PENDING_TTL_MS,
        });
    }
}

/// The single Durable Object holding the ledger.
///
/// One instance (`id_from_name("global")`) so every claim is serialized through
/// the same strongly consistent storage.
#[durable_object]
pub struct DeliveryLedger {
    state: State,
    _env: Env,
}

impl DurableObject for DeliveryLedger {
    fn new(state: State, env: Env) -> Self {
        Self { state, _env: env }
    }

    async fn fetch(&self, mut req: Request) -> Result<Response> {
        let action = req.path().trim_start_matches('/').to_string();
        let keys: Vec<String> = req.json().await?;
        let storage = self.state.storage();
        let mut ledger: Ledger = storage.get("ledger").await?.unwrap_or_default();
        let now_ms = Date::now().as_millis() as i64;

        let out = match action.as_str() {
            "claim" => {
                let granted = ledger.claim(&keys, now_ms);
                ledger.prune(now_ms);
                storage.put("ledger", &ledger).await?;
                granted
            }
            "commit" => {
                ledger.commit(&keys, now_ms);
                ledger.prune(now_ms);
                storage.put("ledger", &ledger).await?;
                keys
            }
            "release" => {
                ledger.release(&keys);
                storage.put("ledger", &ledger).await?;
                keys
            }
            other => {
                return Response::error(format!("unknown ledger action: {other}"), 404);
            }
        };

        Response::from_json(&out)
    }
}

const BINDING: &str = "DELIVERY_LEDGER";

async fn call(env: &Env, action: &str, keys: &[String]) -> Result<Vec<String>> {
    let namespace = env.durable_object(BINDING)?;
    let stub = namespace.id_from_name("global")?.get_stub()?;
    let body = serde_json::to_string(keys)?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_body(Some(JsValue::from_str(&body)));
    let req = Request::new_with_init(&format!("https://delivery-ledger.internal/{action}"), &init)?;
    let mut resp = stub.fetch_with_request(req).await?;
    if resp.status_code() != 200 {
        return Err(format!("ledger {action} failed with {}", resp.status_code()).into());
    }
    resp.json::<Vec<String>>().await
}

/// Claims the given episodes, returning the subset this invocation may announce.
pub async fn claim(env: &Env, keys: &[String]) -> Result<Vec<String>> {
    call(env, "claim", keys).await
}

/// Records confirmed delivery.
pub async fn commit(env: &Env, keys: &[String]) -> Result<Vec<String>> {
    call(env, "commit", keys).await
}

/// Drops claims whose delivery failed, so the next tick retries them.
pub async fn release(env: &Env, keys: &[String]) -> Result<Vec<String>> {
    call(env, "release", keys).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(list: &[(&str, u32)]) -> Vec<String> {
        list.iter().map(|(id, ep)| key(id, *ep)).collect()
    }

    #[test]
    fn first_claim_wins_and_the_duplicate_is_refused() {
        let mut ledger = Ledger::default();
        let k = keys(&[("psyren", 3)]);

        assert_eq!(ledger.claim(&k, 1_000), k, "first caller owns the episode");
        assert!(
            ledger.claim(&k, 1_500).is_empty(),
            "a second invocation minutes later must not announce it again"
        );
    }

    #[test]
    fn a_committed_episode_is_never_claimable_again() {
        let mut ledger = Ledger::default();
        let k = keys(&[("psyren", 3)]);
        ledger.claim(&k, 1_000);
        ledger.commit(&k, 1_200);

        // Long past the pending TTL, still refused: this is the delivery record.
        assert!(ledger.claim(&k, 1_000 + PENDING_TTL_MS * 10).is_empty());
    }

    #[test]
    fn a_failed_delivery_is_released_for_the_next_tick() {
        let mut ledger = Ledger::default();
        let k = keys(&[("psyren", 3)]);
        assert_eq!(ledger.claim(&k, 1_000), k);

        ledger.release(&k);
        assert_eq!(
            ledger.claim(&k, 2_000),
            k,
            "a webhook failure must stay queued, not be suppressed"
        );
    }

    #[test]
    fn an_abandoned_claim_self_heals_after_the_ttl() {
        let mut ledger = Ledger::default();
        let k = keys(&[("psyren", 3)]);
        assert_eq!(ledger.claim(&k, 1_000), k);

        // The worker died between claim and send. No commit, no release.
        assert!(ledger.claim(&k, 1_000 + PENDING_TTL_MS - 1).is_empty());
        assert_eq!(
            ledger.claim(&k, 1_000 + PENDING_TTL_MS),
            k,
            "an abandoned claim must expire or the episode is lost forever"
        );
    }

    #[test]
    fn prunning_drops_old_finals_and_stale_pendings() {
        let mut ledger = Ledger::default();
        let delivered = keys(&[("psyren", 3)]);
        let abandoned = keys(&[("hotaru-no-yomeiri", 4)]);
        ledger.claim(&delivered, 1_000);
        ledger.commit(&delivered, 1_000);
        ledger.claim(&abandoned, 1_000);

        ledger.prune(1_000 + PENDING_TTL_MS);
        assert!(
            ledger.entries.contains_key(&delivered[0]),
            "finals are kept for retention, not pruned with pendings"
        );
        assert!(
            !ledger.entries.contains_key(&abandoned[0]),
            "a stale pending must not sit in storage forever"
        );

        ledger.prune(1_000 + RETENTION_MS + 1);
        assert!(ledger.entries.is_empty(), "retention must bound growth");
    }

    #[test]
    fn distinct_episodes_of_one_title_are_independent() {
        let mut ledger = Ledger::default();
        let ep3 = keys(&[("psyren", 3)]);
        let ep4 = keys(&[("psyren", 4)]);
        ledger.claim(&ep3, 1_000);
        ledger.commit(&ep3, 1_000);

        assert_eq!(
            ledger.claim(&ep4, 2_000),
            ep4,
            "committing one episode must not block the next one"
        );
    }

    #[test]
    fn a_batch_claim_reports_only_the_granted_subset() {
        let mut ledger = Ledger::default();
        let first = keys(&[("psyren", 3)]);
        ledger.claim(&first, 1_000);
        ledger.commit(&first, 1_000);

        let batch = keys(&[("psyren", 3), ("psyren", 4)]);
        assert_eq!(
            ledger.claim(&batch, 2_000),
            keys(&[("psyren", 4)]),
            "already delivered episodes are filtered out of the batch"
        );
    }
}
