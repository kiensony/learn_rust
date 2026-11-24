#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_extensions_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split_ascii_whitespace()
                .map(std::path::Path::new)
                .filter_map(|p| p.extension())
                .map(|e| e.len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
