// rust-track: handwritten task-1900
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1900;

pub fn weak_liveness_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
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
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
