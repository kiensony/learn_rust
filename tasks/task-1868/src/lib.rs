#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn weak_read_handles_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::sync::Arc;
            input
                .iter()
                .map(|&x| {
                    let mut data = Arc::new(x);
                    let _owners: Vec<_> = (0..x % 4).map(|_| Arc::downgrade(&data)).collect();
                    Arc::weak_count(&data) as u32
                })
                .collect()
        };
        samples.reverse();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
