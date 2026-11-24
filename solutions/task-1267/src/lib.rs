// rust-track: handwritten task-1267
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1267;

pub fn positional_options_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_positional_options(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_positional_options(input: &str) -> Vec<u32> {
    let mut cmd = std::process::Command::new("report");
    cmd.args(input.split_ascii_whitespace());
    cmd.get_args()
        .filter(|a| !a.to_str().unwrap().starts_with('-'))
        .map(|a| a.len() as u32)
        .collect()
}
