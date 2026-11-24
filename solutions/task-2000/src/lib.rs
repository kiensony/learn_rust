// rust-track: handwritten task-2000
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2000;

pub fn atomic_exchange_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.swap(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
