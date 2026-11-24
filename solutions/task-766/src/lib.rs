// rust-track: handwritten task-766
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 766;

pub fn closing_marks_removed_slash(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.strip_prefix("path/").unwrap_or(text);
                    text.trim_end_matches('/')
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
