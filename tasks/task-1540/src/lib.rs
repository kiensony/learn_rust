#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packet_gray_code_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x)).collect() };
    samples.into_iter().rev().collect()
}
