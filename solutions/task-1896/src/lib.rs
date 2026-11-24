// rust-track: handwritten task-1896
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1896;

pub fn shared_read_handles_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
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
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
