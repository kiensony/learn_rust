#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn checksum(input: &[u8]) -> u32 {
    input.iter().fold(0u32, |acc, &b| acc ^ u32::from(b))
}
