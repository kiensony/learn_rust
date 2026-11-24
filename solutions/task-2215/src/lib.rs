// rust-track: handwritten task-2215
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2215;

pub fn borrowed_offsets_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples.dedup();
    samples
}
