// rust-track: handwritten task-1852
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1852;

pub fn shared_read_handles_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    samples.into_iter().map(u64::from).sum()
}
