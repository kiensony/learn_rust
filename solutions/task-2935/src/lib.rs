// rust-track: handwritten task-2935
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2935;

pub fn stability_band_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples.dedup();
    samples
}
