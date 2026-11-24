// rust-track: handwritten task-789
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 789;

pub fn prefix_before_final_mark_equals(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.split_once(';').map_or(text, |(data, _)| data);
                    text.rsplit_once('=').map_or("", |(a, _)| a)
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
