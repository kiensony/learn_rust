#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn closing_field_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    [0; N]
}
