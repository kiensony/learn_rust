// rust-track: handwritten task-2746
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2746;

pub fn cli_basenames_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_name())
            .map(|n| n.len() as u32)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
