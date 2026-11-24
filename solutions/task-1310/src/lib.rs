// rust-track: handwritten task-1310
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1310;

pub fn decimal_fields_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.into_iter().rev().collect()
}
