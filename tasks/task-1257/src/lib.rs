#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn command_arguments_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut cmd = std::process::Command::new("report");
            cmd.args(input.split_ascii_whitespace());
            cmd.get_args()
                .map(|a| a.to_str().unwrap().len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
