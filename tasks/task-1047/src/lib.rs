#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn remainder_after_first_mark_hash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    [0; N]
}
