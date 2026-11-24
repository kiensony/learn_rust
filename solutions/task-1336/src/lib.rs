// rust-track: handwritten task-1336
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1336;

pub fn field_indexes_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .enumerate()
            .filter(|(_, x)| *x == 0)
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
