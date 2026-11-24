// rust-track: handwritten task-1496
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1496;

pub fn http_status_fixture_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..600).contains(x))
            .map(|x| x / 100)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
