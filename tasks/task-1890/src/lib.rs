#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn unique_owner_state_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::sync::Arc;
            input
                .iter()
                .map(|&x| {
                    let mut data = Arc::new(x);
                    let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
                    u32::from(Arc::get_mut(&mut data).is_some())
                })
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
