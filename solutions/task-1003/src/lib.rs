// rust-track: handwritten task-1003
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1003;

pub fn closing_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.rsplit(':').next().unwrap_or_default()).len()
    })
}
