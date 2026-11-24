// rust-track: handwritten task-2938
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2938;

pub fn stability_band_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
