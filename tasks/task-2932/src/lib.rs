#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn stability_band_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    let _ = samples;
    0
}
