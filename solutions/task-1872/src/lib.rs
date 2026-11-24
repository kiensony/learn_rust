// rust-track: handwritten task-1872
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1872;

pub fn shared_payload_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
