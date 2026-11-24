// rust-track: handwritten task-796
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 796;

pub fn closing_marks_removed_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    closing_marks_removed_hash_impl(records)
}

fn closing_marks_removed_hash_impl(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
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
                    text.trim_end_matches('#')
                })
                .to_owned(),
            )
            .or_insert(0) += 1;
    }
    counts
}
