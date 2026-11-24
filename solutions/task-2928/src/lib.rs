// rust-track: handwritten task-2928
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2928;

pub fn failure_threshold_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
