// rust-track: handwritten task-956
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 956;

pub fn closing_marks_removed_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.trim_end_matches(':')).len())
        .collect()
}
