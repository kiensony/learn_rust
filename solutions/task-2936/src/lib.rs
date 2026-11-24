// rust-track: handwritten task-2936
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2936;

pub fn stability_band_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
