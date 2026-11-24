// rust-track: handwritten task-2736
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2736;

pub fn cli_rust_sources_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .enumerate()
            .filter(|(_, p)| p.extension().is_some_and(|e| e == "rs"))
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
