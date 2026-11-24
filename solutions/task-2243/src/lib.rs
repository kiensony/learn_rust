// rust-track: handwritten task-2243
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2243;

pub fn borrowed_windows_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .as_bytes()
            .windows(2)
            .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
