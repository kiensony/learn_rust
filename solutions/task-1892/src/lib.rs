// rust-track: handwritten task-1892
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1892;

pub fn weak_liveness_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
