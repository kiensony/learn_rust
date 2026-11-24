// rust-track: handwritten task-2208
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2208;

pub fn borrowed_prefixes_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
