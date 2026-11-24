#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packed_booleans_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| x.count_ones())
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
