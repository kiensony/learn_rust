// rust-track: handwritten task-1332
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1332;

pub fn field_indexes_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .enumerate()
            .filter(|(_, x)| *x == 0)
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
