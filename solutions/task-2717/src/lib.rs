// rust-track: handwritten task-2717
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2717;

pub fn cli_extensions_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_cli_extensions(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_cli_extensions(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(std::path::Path::new)
        .filter_map(|p| p.extension())
        .map(|e| e.len() as u32)
        .collect()
}
