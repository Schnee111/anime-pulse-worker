//! Per-stage tick timing.
//!
//! `cpuTime` on the Workers Free tier has been blamed twice on the wrong thing
//! (the 590 KB Sokuja payload, then the 2.66 MB WASM module), and both guesses
//! were refuted by measurement. This exists so the next answer is measured
//! instead of argued: each stage records elapsed milliseconds and the tick
//! emits a single structured line that `wrangler tail` can be grepped for.
//!
//! Costs are whole milliseconds from `Date::now()`, which is coarse but free. The
//! instrumentation must never add a network call or allocate per stage.

use serde::Serialize;
use worker::*;

/// Elapsed milliseconds per stage of one scheduled tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TickProfile {
    /// Reading `tracked_anime` out of KV.
    pub kv_read: i64,
    /// Samehadaku JSON fetch + parse.
    pub samehadaku: i64,
    /// Telegram channel fetch + parse, or 0 when the throttle skipped it.
    pub telegram: i64,
    /// Candidate-versus-watchlist matching loop.
    pub matching: i64,
    /// Wall time for the whole tick.
    pub total: i64,
}

/// Current epoch milliseconds.
pub fn now_ms() -> i64 {
    Date::now().as_millis() as i64
}

impl TickProfile {
    /// Records the elapsed time since `start_ms` into `slot`.
    ///
    /// Clamped at zero: `saturating_sub` bottoms out at `i64::MIN`, not 0, so a
    /// clock that steps backwards would otherwise log a negative cost.
    pub fn record(&mut self, slot: Stage, start_ms: i64, end_ms: i64) {
        let elapsed = end_ms.saturating_sub(start_ms).max(0);
        match slot {
            Stage::KvRead => self.kv_read = elapsed,
            Stage::Samehadaku => self.samehadaku = elapsed,
            Stage::Telegram => self.telegram = elapsed,
            Stage::Matching => self.matching = elapsed,
        }
    }

    /// Emits the single greppable line for this tick.
    pub fn log(&self) {
        console_log!(
            "PROFILE kv_read={} samehadaku={} telegram={} matching={} total={}",
            self.kv_read,
            self.samehadaku,
            self.telegram,
            self.matching,
            self.total
        );
    }

    /// Sum of the instrumented stages, so the log can show how much of `total`
    /// is actually accounted for. Unaccounted time points at runtime overhead
    /// (module init, serialisation) rather than our own code.
    pub fn accounted(&self) -> i64 {
        self.kv_read + self.samehadaku + self.telegram + self.matching
    }
}

/// Which stage a `record` call belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    KvRead,
    Samehadaku,
    Telegram,
    Matching,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_elapsed_per_stage() {
        let mut p = TickProfile::default();
        p.record(Stage::KvRead, 1000, 1008);
        p.record(Stage::Samehadaku, 1008, 1050);
        p.record(Stage::Matching, 1050, 1051);
        assert_eq!(p.kv_read, 8);
        assert_eq!(p.samehadaku, 42);
        assert_eq!(p.matching, 1);
        assert_eq!(p.telegram, 0);
        assert_eq!(p.accounted(), 51);
    }

    #[test]
    fn backwards_clock_saturates_instead_of_going_negative() {
        let mut p = TickProfile::default();
        p.record(Stage::Telegram, 2000, 1900);
        assert_eq!(p.telegram, 0);
    }

    #[test]
    fn serialises_for_the_trigger_response() {
        let p = TickProfile {
            kv_read: 3,
            samehadaku: 40,
            telegram: 0,
            matching: 2,
            total: 60,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"samehadaku\":40"));
        assert!(json.contains("\"telegram\":0"));
        assert!(json.contains("\"total\":60"));
    }
}
