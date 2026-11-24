// rust-track: handwritten task-1007
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1007;

pub fn remainder_after_first_mark_colon<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.split_once(':').map_or("", |(_, b)| b)).len()
    })
}
