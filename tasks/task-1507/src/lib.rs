#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packet_high_nibble_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
