// rust-track: handwritten task-2718
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2718;

pub fn cli_extensions_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
