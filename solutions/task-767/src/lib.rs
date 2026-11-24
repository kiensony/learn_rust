// rust-track: handwritten task-767
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 767;

pub fn marks_removed_at_both_ends_slash(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.strip_prefix("path/").unwrap_or(text);
                    text.trim_matches('/')
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
