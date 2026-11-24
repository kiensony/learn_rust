// rust-track: handwritten task-2383
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2383;

pub fn rolling_parity_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut p = 0;
        input
            .iter()
            .map(|&x| {
                p ^= x;
                u32::from(p)
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
