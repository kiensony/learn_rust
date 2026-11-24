// rust-track: handwritten task-1884
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1884;

pub fn unique_owner_state_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_unique_owner_state(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_unique_owner_state(input: &[u8]) -> Vec<u32> {
    use std::sync::Arc;
    input
        .iter()
        .map(|&x| {
            let mut data = Arc::new(x);
            let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
            u32::from(Arc::get_mut(&mut data).is_some())
        })
        .collect()
}
