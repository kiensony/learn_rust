// rust-track: handwritten task-2932
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2932;

pub fn stability_band_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
