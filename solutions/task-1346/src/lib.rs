// rust-track: handwritten task-1346
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1346;

pub fn decimal_fields_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
