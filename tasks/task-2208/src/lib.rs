#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_prefixes_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    let _ = samples;
    Vec::new()
}
