// rust-track: handwritten task-760
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 760;

pub fn first_nonempty_field_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry((text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).to_owned())
            .or_insert(0) += 1;
    }
    counts
}
