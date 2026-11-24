#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_hash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    Vec::new()
}
