// rust-track: handwritten task-953
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 953;

pub fn leading_field_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.split(':').next().unwrap_or_default()).len())
        .collect()
}
