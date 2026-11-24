// rust-track: handwritten task-2707
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2707;

pub fn cli_basenames_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_cli_basenames(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_cli_basenames(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(std::path::Path::new)
        .filter_map(|p| p.file_name())
        .map(|n| n.len() as u32)
        .collect()
}
