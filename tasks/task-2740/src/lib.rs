#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cli_rust_sources_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split_ascii_whitespace()
                .map(std::path::Path::new)
                .enumerate()
                .filter(|(_, p)| p.extension().is_some_and(|e| e == "rs"))
                .map(|(i, _)| i as u32)
                .collect()
        };
        samples.reverse();
        samples
    };
    samples.into_iter().rev().collect()
}
