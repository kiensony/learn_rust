#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn failure_threshold_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
            .collect()
    };
    let _ = samples;
    None
}
