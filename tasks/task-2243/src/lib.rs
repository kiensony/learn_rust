#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_windows_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .as_bytes()
                .windows(2)
                .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
