#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn field_indexes_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .enumerate()
            .filter(|(_, x)| *x == 0)
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
