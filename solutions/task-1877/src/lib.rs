// rust-track: handwritten task-1877
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1877;

pub fn shared_payload_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::sync::Arc;
        input
            .iter()
            .map(|&x| {
                let mut data = Arc::new(x);
                let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
                u32::from(*data)
            })
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
