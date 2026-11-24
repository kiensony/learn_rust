// rust-track: handwritten task-1011
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1011;

pub fn shortest_field_with_first_tie_colon<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len()
    })
}
