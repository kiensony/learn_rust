// rust-track: handwritten task-755
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 755;

pub fn closing_field_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for &text in records {
        *counts
            .entry((text.rsplit(':').next().unwrap_or_default()).to_owned())
            .or_insert(0) += 1;
    }
    counts
}
