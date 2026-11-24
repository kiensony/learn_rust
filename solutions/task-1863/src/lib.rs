// rust-track: handwritten task-1863
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1863;

pub fn weak_read_handles_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::sync::Arc;
        input
            .iter()
            .map(|&x| {
                let mut data = Arc::new(x);
                let _owners: Vec<_> = (0..x % 4).map(|_| Arc::downgrade(&data)).collect();
                Arc::weak_count(&data) as u32
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
