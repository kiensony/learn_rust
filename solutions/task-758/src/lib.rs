// rust-track: handwritten task-758
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 758;

pub fn marks_removed_at_both_ends_colon(
    records: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry((text.trim_matches(':')).to_owned())
            .or_insert(0) += 1;
    }
    counts
}
