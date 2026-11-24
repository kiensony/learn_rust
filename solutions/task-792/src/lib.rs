// rust-track: handwritten task-792
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 792;

pub fn shortest_field_with_first_tie_equals(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.split_once(';').map_or(text, |(data, _)| data);
                    text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
