// rust-track: handwritten task-1883
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1883;

pub fn unique_owner_state_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
