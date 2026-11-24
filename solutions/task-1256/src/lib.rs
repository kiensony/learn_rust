// rust-track: handwritten task-1256
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1256;

pub fn command_arguments_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .map(|a| a.to_str().unwrap().len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
