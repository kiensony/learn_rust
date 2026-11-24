// rust-track: handwritten task-954
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 954;

pub fn closing_field_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.rsplit(':').next().unwrap_or_default()).len())
        .collect()
}
