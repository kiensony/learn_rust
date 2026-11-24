#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_register_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.windows(2).map(|c| u32::from(c[0] & c[1])).collect() };
    samples
}
