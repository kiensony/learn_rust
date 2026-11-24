// rust-track: handwritten task-1500
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1500;

pub fn status_detail_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|x| (100..600).contains(x))
            .map(|x| x % 100)
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
