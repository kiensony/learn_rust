// rust-track: handwritten task-2711
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2711;

pub fn cli_extensions_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
    };
    samples
}
