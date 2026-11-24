// rust-track: handwritten task-2710
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2710;

pub fn cli_basenames_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_name())
            .map(|n| n.len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
