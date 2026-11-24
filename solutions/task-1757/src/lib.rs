// rust-track: handwritten task-1757
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1757;

pub fn build_multiplier_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_build_multiplier(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_build_multiplier(input: &[u8]) -> Vec<u32> {
    let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
    input.iter().map(|&x| u32::from(x) * units).collect()
}
