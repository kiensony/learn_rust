// rust-track: handwritten task-1307
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1307;

pub fn decimal_fields_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_decimal_fields(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_decimal_fields(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .map(u32::from)
        .collect()
}
