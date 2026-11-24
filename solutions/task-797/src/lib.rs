// rust-track: handwritten task-797
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 797;

pub fn marks_removed_at_both_ends_hash(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text
                        .split_once('[')
                        .and_then(|(_, tail)| tail.split_once(']'))
                        .map_or("", |(inside, _)| inside);
                    text.trim_matches('#')
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
