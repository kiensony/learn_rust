#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_scalars_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
