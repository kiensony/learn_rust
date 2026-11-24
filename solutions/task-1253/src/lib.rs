// rust-track: handwritten task-1253
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1253;

pub fn command_arguments_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .map(|a| a.to_str().unwrap().len() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
