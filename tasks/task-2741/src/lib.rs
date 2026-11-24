#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_stems_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_stem())
            .map(|n| n.len() as u32)
            .collect()
    };
    let _ = samples;
    Vec::new()
}
