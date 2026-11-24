// rust-track: handwritten task-2217
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2217;

pub fn borrowed_offsets_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_borrowed_offsets(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_borrowed_offsets(input: &str) -> Vec<u32> {
    input.match_indices('\n').map(|(i, _)| i as u32).collect()
}
