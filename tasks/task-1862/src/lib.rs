#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn weak_read_handles_total(input: &[u8]) -> u64 {
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
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().map(u64::from).sum()
}
