// rust-track: handwritten task-1006
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1006;

pub fn marks_removed_at_both_ends_colon<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.trim_matches(':')).len()
    })
}
