// rust-track: handwritten task-1856
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1856;

pub fn shared_read_handles_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
