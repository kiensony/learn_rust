#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn first_nonempty_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    [0; N]
}
