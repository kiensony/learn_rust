// rust-track: handwritten task-2750
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2750;

pub fn cli_stems_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.file_stem())
            .map(|n| n.len() as u32)
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
