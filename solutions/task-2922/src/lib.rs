// rust-track: handwritten task-2922
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2922;

pub fn failure_threshold_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
