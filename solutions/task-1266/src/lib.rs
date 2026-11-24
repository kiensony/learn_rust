// rust-track: handwritten task-1266
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1266;

pub fn positional_options_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter(|a| !a.to_str().unwrap().starts_with('-'))
            .map(|a| a.len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
