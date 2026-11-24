// rust-track: handwritten task-1866
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1866;

pub fn weak_read_handles_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
