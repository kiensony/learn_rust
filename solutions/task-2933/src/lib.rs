// rust-track: handwritten task-2933
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2933;

pub fn stability_band_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
