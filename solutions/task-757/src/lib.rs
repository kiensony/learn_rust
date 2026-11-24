// rust-track: handwritten task-757
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 757;

pub fn closing_marks_removed_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry((text.trim_end_matches(':')).to_owned())
            .or_insert(0) += 1;
    }
    counts
}
