// rust-track: handwritten task-957
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 957;

pub fn marks_removed_at_both_ends_colon<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.trim_matches(':')).len())
        .collect()
}
