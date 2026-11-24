#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn weak_liveness_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::Arc;
        input
            .iter()
            .map(|&x| {
                let data = Arc::new(x);
                let weak = Arc::downgrade(&data);
                let _other = (x % 2 == 1).then(|| Arc::clone(&data));
                drop(data);
                u32::from(weak.upgrade().is_some())
            })
            .collect()
    };
    let _ = samples;
    Vec::new()
}
