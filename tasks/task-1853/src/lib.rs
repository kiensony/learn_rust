#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shared_read_handles_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::sync::Arc;
            input
                .iter()
                .map(|&x| {
                    let mut data = Arc::new(x);
                    let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
                    Arc::strong_count(&data) as u32
                })
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
