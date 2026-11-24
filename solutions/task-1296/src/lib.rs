// rust-track: handwritten task-1296
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1296;

pub fn command_arguments_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .map(|a| a.to_str().unwrap().len() as u32)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
