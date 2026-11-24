// rust-track: handwritten task-2930
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2930;

pub fn failure_threshold_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    samples.into_iter().rev().collect()
}
