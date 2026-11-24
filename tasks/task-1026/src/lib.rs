#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn marks_removed_at_both_ends_pipe<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    [0; N]
}
