// rust-track: handwritten task-2228
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2228;

pub fn borrowed_scalars_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
