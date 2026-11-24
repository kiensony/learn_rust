#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_stems_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split_ascii_whitespace()
                .map(std::path::Path::new)
                .filter_map(|p| p.file_stem())
                .map(|n| n.len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
