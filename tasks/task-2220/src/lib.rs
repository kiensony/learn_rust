#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_offsets_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    let _ = samples;
    Vec::new()
}
