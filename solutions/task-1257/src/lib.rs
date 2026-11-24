// rust-track: handwritten task-1257
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1257;

pub fn command_arguments_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_command_arguments(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_command_arguments(input: &str) -> Vec<u32> {
    let mut cmd = std::process::Command::new("report");
    cmd.args(input.split_ascii_whitespace());
    cmd.get_args()
        .map(|a| a.to_str().unwrap().len() as u32)
        .collect()
}
