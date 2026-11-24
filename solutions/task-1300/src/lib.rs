// rust-track: handwritten task-1300
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1300;

pub fn quiet_option_positions_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .enumerate()
            .filter(|(_, a)| *a == "-q")
            .map(|(i, _)| i as u32)
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
