// rust-track: handwritten task-2745
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2745;

pub fn cli_stems_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_stem())
            .map(|n| n.len() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
