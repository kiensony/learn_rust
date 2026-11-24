// rust-track: handwritten task-1857
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1857;

pub fn shared_read_handles_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_shared_read_handles(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_shared_read_handles(input: &[u8]) -> Vec<u32> {
    use std::sync::Arc;
    input
        .iter()
        .map(|&x| {
            let mut data = Arc::new(x);
            let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
            Arc::strong_count(&data) as u32
        })
        .collect()
}
