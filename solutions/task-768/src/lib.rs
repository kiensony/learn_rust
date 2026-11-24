// rust-track: handwritten task-768
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 768;

pub fn remainder_after_first_mark_slash(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.strip_prefix("path/").unwrap_or(text);
                    text.split_once('/').map_or("", |(_, b)| b)
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
