// rust-track: handwritten task-1252
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1252;

pub fn command_arguments_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .map(|a| a.to_str().unwrap().len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
