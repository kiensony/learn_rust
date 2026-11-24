// rust-track: handwritten task-1767
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1767;

pub fn generated_offset_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_generated_offset(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_generated_offset(input: &[u8]) -> Vec<u32> {
    let units = env!("TRACK_UNITS").parse::<u32>().unwrap();
    input.iter().map(|&x| u32::from(x) + units).collect()
}
