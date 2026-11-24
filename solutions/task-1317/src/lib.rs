// rust-track: handwritten task-1317
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1317;

pub fn nibble_fields_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_nibble_fields(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_nibble_fields(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .map(|x| u32::from(x >> 4))
        .collect()
}
