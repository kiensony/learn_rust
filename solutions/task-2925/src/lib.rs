// rust-track: handwritten task-2925
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2925;

pub fn failure_threshold_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    samples.dedup();
    samples
}
