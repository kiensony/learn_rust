// rust-track: handwritten task-2702
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2702;

pub fn cli_basenames_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_name())
            .map(|n| n.len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
