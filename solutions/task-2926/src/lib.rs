// rust-track: handwritten task-2926
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2926;

pub fn failure_threshold_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
