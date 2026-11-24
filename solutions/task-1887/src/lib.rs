// rust-track: handwritten task-1887
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1887;

pub fn unique_owner_state_distinct(input: &[u8]) -> Vec<u32> {
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
    samples.sort_unstable();
    samples.dedup();
    samples
}
