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

/// Matches tracked anime title against candidate title using word token overlap.
/// Handles differences like "Season 2" vs "2nd Season" or subtitle suffixes.
pub fn matches_tracked(tracked_title: &str, candidate_title: &str) -> bool {
    let re_word = Regex::new(r"(?i)[a-z0-9]{3,}").unwrap();

    let words_tracked: HashSet<String> = re_word
        .find_iter(tracked_title)
        .map(|m| m.as_str().to_lowercase())
        .collect();

    let words_candidate: HashSet<String> = re_word
        .find_iter(candidate_title)
        .map(|m| m.as_str().to_lowercase())
        .collect();

    let overlap = words_tracked.intersection(&words_candidate).count();

    // If tracked has 1-2 words, need at least 1 match. If 3+, need at least 2 matching words.
    if words_tracked.len() <= 2 {
        overlap >= 1
    } else {
        overlap >= 2
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
