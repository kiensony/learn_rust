// rust-track: handwritten task-771
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 771;

pub fn longest_field_with_last_tie_slash(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.strip_prefix("path/").unwrap_or(text);
                    text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
