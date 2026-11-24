// rust-track: handwritten task-2738
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2738;

pub fn cli_rust_sources_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .enumerate()
            .filter(|(_, p)| p.extension().is_some_and(|e| e == "rs"))
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
