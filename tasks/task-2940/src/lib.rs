#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn stability_band_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .rchunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples.into_iter().rev().collect()
}
