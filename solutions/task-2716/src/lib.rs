// rust-track: handwritten task-2716
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2716;

pub fn cli_extensions_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
