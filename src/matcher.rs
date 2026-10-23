use regex::Regex;
use std::collections::HashSet;

/// Extracts title and episode number from fansub post title.
/// Example: "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu. Episode 2 Subtitle Indonesia"
/// Returns: Some(("Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu.", 2))
pub fn parse_release_title(raw_title: &str) -> Option<(String, u32)> {
    let re = Regex::new(r"(?i)^(.*?)\s+Episode\s+(\d+)").ok()?;
    let caps = re.captures(raw_title)?;
    let title = caps.get(1)?.as_str().trim().to_string();
    let episode = caps.get(2)?.as_str().parse::<u32>().ok()?;
    Some((title, episode))
}

/// Minimum share of a tracked title's keywords that a candidate must share, and
/// the shortest distinctive token that may carry a match on its own.
///
/// Romanised Japanese titles lean hard on particles, so a pure overlap count is
/// not evidence of anything: "Chitose-kun wa Ramune Bin no Naka Part 2" and
/// "Ao no Hako Season 2" share `{2, no}` and nothing else, yet cleared the old
/// 2-token rule. Measured against a live feed snapshot (Samehadaku 20 posts +
/// Telegram Sokuja 20 posts, 21 tracked titles), the previous rule produced 16
/// such cross-title matches; these thresholds reduce that to 0 without dropping
/// any of the 12 legitimate matches.
///
/// The bar is a **strict** majority, not `>= 0.5`: with `>= 0.5` a 2-keyword
/// title still matched on a single shared word ("Romelia Senki" vs "Youjo
/// Senki"), which is exactly the failure this change is meant to remove.
const MIN_ANCHOR_LEN: usize = 4;

fn stop_words() -> HashSet<&'static str> {
    [
        "season",
        "2nd",
        "3rd",
        "4th",
        "5th",
        "second",
        "third",
        "fourth",
        "fifth",
        "part",
        "sub",
        "indo",
        "subtitle",
        "indonesia",
        "episode",
        "the",
        "and",
        "cour",
        // Particles and filler that appear in almost every romanised title. They
        // carry no series identity, and counting them is what let unrelated
        // titles match each other on `no` + `2` alone.
        "no",
        "wa",
        "ga",
        "ni",
        "wo",
        "de",
        "to",
        "na",
        "mo",
        "ka",
        "yo",
        "ne",
        "kara",
        "made",
        "dake",
        "suru",
        "shita",
        "ore",
        "sore",
        "kore",
        "are",
        "desu",
        "masu",
        "aru",
        "iru",
        "koto",
        "mono",
        "naka",
        "hito",
        "you",
        "sama",
        "san",
        "kun",
        "chan",
    ]
    .iter()
    .copied()
    .collect()
}

fn extract_significant_keywords(title: &str) -> HashSet<String> {
    let re = Regex::new(r"(?i)[a-z0-9]+").unwrap();
    let stops = stop_words();
    re.find_iter(title)
        .map(|m| m.as_str().to_lowercase())
        .filter(|w| !stops.contains(w.as_str()))
        .collect()
}

/// Matches tracked anime title against candidate title using keyword token overlap.
///
/// Filters generic terms (`season`, `part`, `sub`, and the Japanese particles
/// that dominate romanised titles), then requires a strict majority of the
/// tracked title's keywords plus at least one shared token long enough to be
/// distinctive. A bare overlap count is not enough: two unrelated sequels share
/// `{2}` plus a particle and nothing else.
pub fn matches_tracked(tracked_title: &str, candidate_title: &str) -> bool {
    let kw_tracked = extract_significant_keywords(tracked_title);
    let kw_candidate = extract_significant_keywords(candidate_title);

    let overlap: HashSet<&String> = kw_tracked.intersection(&kw_candidate).collect();
    if overlap.is_empty() {
        return false;
    }

    if kw_tracked.len() <= 1 {
        return true;
    }

    // A full keyword match is decisive on its own. Without this short-circuit a
    // title made only of short words (`Eat-Man`, `Run Dim`) could never pass the
    // anchor check even when every keyword matched.
    if overlap.len() == kw_tracked.len() {
        return true;
    }

    // Strict majority: `overlap * 2 > tracked`. For a 2-keyword title that means
    // both words, so `Romelia Senki` no longer matches `Youjo Senki` on `senki`.
    let is_strict_majority = overlap.len() * 2 > kw_tracked.len();
    let has_anchor = overlap.iter().any(|w| w.chars().count() >= MIN_ANCHOR_LEN);

    is_strict_majority && has_anchor
}

/// Shortens a title with an ellipsis if it exceeds max_len.
pub fn short_title(title: &str, max_len: usize) -> String {
    let trimmed = title.trim();
    if trimmed.chars().count() <= max_len {
        trimmed.to_string()
    } else {
        let mut s: String = trimmed.chars().take(max_len - 1).collect();
        s.push('…');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_release_title() {
        let sample = "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu. Episode 2 Subtitle Indonesia";
        let res = parse_release_title(sample);
        assert!(res.is_some());
        let (title, ep) = res.unwrap();
        assert_eq!(
            title,
            "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu."
        );
        assert_eq!(ep, 2);

        let sample2 = "Ao no Hako Season 2 Episode 1 Subtitle Indonesia";
        let (title2, ep2) = parse_release_title(sample2).unwrap();
        assert_eq!(title2, "Ao no Hako Season 2");
        assert_eq!(ep2, 1);
    }

    #[test]
    fn test_matches_tracked() {
        assert!(matches_tracked(
            "Yasei no Last Boss ga Arawareta! Season 2",
            "Yasei no Last Boss ga Arawareta! 2nd Season"
        ));
        assert!(matches_tracked(
            "Koori no Jouheki Season 2",
            "Koori no Jouheki 2nd Season"
        ));
        assert!(matches_tracked(
            "Tempal: Item no Chikara",
            "Tempal: Item no Chikara"
        ));
        assert!(matches_tracked(
            "FX Senshi Kurumi-chan",
            "FX Senshi Kurumi-chan"
        ));
        assert!(matches_tracked(
            "Ao no Hako Season 2",
            "Ao no Hako 2nd Season"
        ));
        assert!(!matches_tracked(
            "Ao no Hako Season 2",
            "Tensei Kizoku, Kantei Skill de Nariagaru 3rd Season"
        ));
        assert!(!matches_tracked(
            "Koori no Jouheki Season 2",
            "Tokyo Revengers Season 4"
        ));
    }

    /// Regression for issue #29. Each pair was a live false match taken from a
    /// real feed snapshot (Samehadaku 20 posts + Telegram Sokuja 20 posts
    /// against 21 tracked titles, 2026-10-23): the candidate release was being
    /// attributed to a completely unrelated tracked title.
    #[test]
    fn unrelated_titles_sharing_only_particles_do_not_match() {
        let cases = [
            // Chitose-kun was claimed by both Koori and Ao no Hako on {2, no}.
            (
                "Ao no Hako Season 2",
                "Chitose-kun wa Ramune Bin no Naka Part 2",
            ),
            (
                "Koori no Jouheki Season 2",
                "Chitose-kun wa Ramune Bin no Naka Part 2",
            ),
            (
                "Tantei wa Mou, Shindeiru. Season 2",
                "Chitose-kun wa Ramune Bin no Naka Part 2",
            ),
            // Kikansha was claimed by three unrelated sequels.
            (
                "Ao no Hako Season 2",
                "Kikansha no Mahou wa Tokubetsu desu Season 2",
            ),
            (
                "Koori no Jouheki Season 2",
                "Kikansha no Mahou wa Tokubetsu desu Season 2",
            ),
            (
                "Tantei wa Mou, Shindeiru. Season 2",
                "Kikansha no Mahou wa Tokubetsu desu Season 2",
            ),
            // Tsuihou Cheat was claimed on {dake, ni, no, ore} + a shared verb.
            (
                "Shiotaiou no Satou-san ga Ore ni dake Amai",
                "Tsuihou sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka suru",
            ),
            (
                "Doumo, Suki na Hito ni Horegusuri wo Irai sareta Majo desu.",
                "Tsuihou sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka suru",
            ),
            // Hyouken was claimed by Sekai Saikyou on {no, sekai}.
            (
                "Sekai Saikyou no Majo, Hajimemashita",
                "Hyouken no Majutsushi ga Sekai wo Suberu II",
            ),
            (
                "Hyouken no Majutsushi ga Sekai wo Suberu Season 2",
                "Tsuihou sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka suru",
            ),
        ];

        for (tracked, candidate) in cases {
            assert!(
                !matches_tracked(tracked, candidate),
                "expected NO match: tracked={tracked:?} candidate={candidate:?}"
            );
        }
    }

    /// Regression for issue #29, other direction: the same snapshot's genuine
    /// matches must survive the stricter rule.
    #[test]
    fn genuine_matches_from_the_same_snapshot_still_match() {
        let cases = [
            (
                "Hyouken no Majutsushi ga Sekai wo Suberu Season 2",
                "Hyouken no Majutsushi ga Sekai wo Suberu II",
            ),
            ("Koori no Jouheki Season 2", "Koori no Jouheki 2nd Season"),
            ("FX Senshi Kurumi-chan", "FX Senshi Kurumi-chan"),
            (
                "Tantei wa Mou, Shindeiru. Season 2",
                "Tantei wa Mou, Shindeiru. Season 2",
            ),
            (
                "Sekai Saikyou no Majo, Hajimemashita",
                "Sekai Saikyou no Majo, Hajimemashita",
            ),
            (
                "Chitose-kun wa Ramune Bin no Naka Part 2",
                "Chitose-kun wa Ramune Bin no Naka Part 2",
            ),
            (
                "Tsuihou Sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka Suru",
                "Tsuihou sareta Cheat Fuyo Majutsushi wa Kimama na Second Life wo Ouka suru. Ore wa Buki dake ja Naku, Arayuru Mono ni \"Kyouka Point\" wo Fuyo Dekiru",
            ),
            (
                "Shiotaiou no Satou-san ga Ore ni dake Amai",
                "Shiotaiou no Satou-san ga Ore ni dake Amai",
            ),
            (
                "Yasei no Last Boss ga Arawareta! Season 2",
                "Yasei no Last Boss ga Arawareta! 2nd Season",
            ),
            ("Tempal: Item no Chikara", "Tempal: Item no Chikara"),
            ("Romelia Senki", "Romelia Senki"),
            ("Kyouran Reijou Nia Liston", "Kyouran Reijou Nia Liston"),
        ];

        for (tracked, candidate) in cases {
            assert!(
                matches_tracked(tracked, candidate),
                "expected a match: tracked={tracked:?} candidate={candidate:?}"
            );
        }
    }

    #[test]
    fn test_short_title() {
        assert_eq!(short_title("Short Title", 20), "Short Title");
        assert_eq!(
            short_title(
                "Tenkou-saki no Seiso Karen na Bishoujo ga, Mukashi Danshi",
                25
            ),
            "Tenkou-saki no Seiso Kar…"
        );
    }

    /// Review follow-up on issue #29. A `>= 0.5` coverage threshold still let a
    /// 2-keyword title match on one shared word, so the bar is a strict majority.
    #[test]
    fn two_keyword_title_does_not_match_on_one_shared_word() {
        assert!(!matches_tracked("Romelia Senki", "Youjo Senki"));
        assert!(!matches_tracked("Romelia Senki", "Senki to Yobareta Otoko"));
        // Both words still match.
        assert!(matches_tracked("Romelia Senki", "Romelia Senki"));
    }

    /// Review follow-up on issue #29. The anchor rule must not reject a title
    /// that only contains short words: a full keyword match is decisive.
    #[test]
    fn exact_match_of_short_words_is_accepted() {
        assert!(matches_tracked("Eat-Man", "Eat-Man"));
        assert!(matches_tracked("Run Dim", "Run Dim"));
        // A different short-word title must still be rejected.
        assert!(!matches_tracked("Eat-Man", "Run Dim"));
    }
}
