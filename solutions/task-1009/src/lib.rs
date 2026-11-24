// rust-track: handwritten task-1009
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1009;

pub fn first_nonempty_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len()
    })
}
