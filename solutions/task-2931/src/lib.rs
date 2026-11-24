// rust-track: handwritten task-2931
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2931;

pub fn stability_band_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
            .collect()
    };
    samples
}
