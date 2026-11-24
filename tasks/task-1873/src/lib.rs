#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn shared_payload_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    let _ = samples;
    None
}
