// rust-track: handwritten task-2320
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2320;

pub fn paired_speedup_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[0].saturating_sub(p[1])))
            .collect()
    };
    samples.into_iter().rev().collect()
}
