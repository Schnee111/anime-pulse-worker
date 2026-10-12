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

fn stop_words() -> HashSet<&'static str> {
    [
        "season",
        "2nd",
        "3rd",
        "4th",
        "5th",
        "part",
        "sub",
        "indo",
        "subtitle",
        "indonesia",
        "episode",
        "the",
        "and",
        "cour",
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
/// Filters out generic terms like "season", "2nd", "3rd", "sub", "indo".
pub fn matches_tracked(tracked_title: &str, candidate_title: &str) -> bool {
    let kw_tracked = extract_significant_keywords(tracked_title);
    let kw_candidate = extract_significant_keywords(candidate_title);

    let overlap = kw_tracked.intersection(&kw_candidate).count();
    if overlap == 0 {
        return false;
    }

    if kw_tracked.len() <= 1 {
        overlap >= 1
    } else if kw_tracked.len() == 2 {
        overlap >= 2
    } else {
        // For longer titles (3+ keywords), require at least 2 overlapping keywords
        // and at least 35% overlap of the tracked keywords.
        overlap >= 2 && ((overlap as f32 / kw_tracked.len() as f32) >= 0.35)
    }
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
}
