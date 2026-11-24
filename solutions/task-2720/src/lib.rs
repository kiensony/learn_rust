// rust-track: handwritten task-2720
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2720;

pub fn cli_extensions_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
