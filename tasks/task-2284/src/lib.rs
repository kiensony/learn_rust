#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn find_newlines(input: &[u8]) -> usize {
    input.iter().position(|&b| b == b'\n').map_or(0, |_| 1)
}
