// rust-track: handwritten task-2943
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2943;

pub fn rollback_triggers_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(u16::from(p[1]) > 2 * u16::from(p[0]))))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
