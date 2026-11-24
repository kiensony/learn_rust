// rust-track: handwritten task-2211
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2211;

pub fn borrowed_offsets_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples
}
