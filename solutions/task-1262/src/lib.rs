// rust-track: handwritten task-1262
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1262;

pub fn positional_options_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter(|a| !a.to_str().unwrap().starts_with('-'))
            .map(|a| a.len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
