#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shortest_field_with_first_tie_slash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    [0; N]
}
