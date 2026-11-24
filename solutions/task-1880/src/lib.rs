// rust-track: handwritten task-1880
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1880;

pub fn shared_payload_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
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
    samples.into_iter().rev().collect()
}
