// rust-track: handwritten task-2905
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2905;

pub fn latency_increases_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.dedup();
    samples
}
