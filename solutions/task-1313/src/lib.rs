// rust-track: handwritten task-1313
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1313;

pub fn nibble_fields_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
