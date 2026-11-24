#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_scalars_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    let _ = samples;
    None
}
