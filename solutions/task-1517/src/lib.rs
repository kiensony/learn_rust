// rust-track: handwritten task-1517
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1517;

pub fn packet_low_nibble_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_packet_low_nibble(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_packet_low_nibble(input: &[u8]) -> Vec<u32> {
    input.iter().map(|&x| u32::from(wire::decode(x))).collect()
}
