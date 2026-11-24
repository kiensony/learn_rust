// rust-track: handwritten task-2715
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2715;

pub fn cli_extensions_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
