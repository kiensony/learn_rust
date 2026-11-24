// rust-track: handwritten task-1871
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1871;

pub fn shared_payload_trace(input: &[u8]) -> Vec<u32> {
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
    samples
}
