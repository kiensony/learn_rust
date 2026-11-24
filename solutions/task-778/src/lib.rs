// rust-track: handwritten task-778
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 778;

pub fn remainder_after_first_mark_pipe(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry(
                ({
                    let text: &str = &text;
                    let text = text.lines().last().unwrap_or_default();
                    text.split_once('|').map_or("", |(_, b)| b)
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
