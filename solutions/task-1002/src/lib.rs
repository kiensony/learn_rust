// rust-track: handwritten task-1002
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1002;

pub fn leading_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.split(':').next().unwrap_or_default()).len()
    })
}
