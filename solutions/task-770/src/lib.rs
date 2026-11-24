// rust-track: handwritten task-770
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 770;

pub fn first_nonempty_field_slash(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.strip_prefix("path/").unwrap_or(text);
                    text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
