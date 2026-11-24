// rust-track: handwritten task-2703
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2703;

pub fn cli_basenames_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_name())
            .map(|n| n.len() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
