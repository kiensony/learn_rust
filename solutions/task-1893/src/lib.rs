// rust-track: handwritten task-1893
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1893;

pub fn weak_liveness_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
