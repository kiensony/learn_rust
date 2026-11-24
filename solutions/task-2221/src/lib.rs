// rust-track: handwritten task-2221
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2221;

pub fn borrowed_scalars_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples
}
