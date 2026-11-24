#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn positional_options_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut cmd = std::process::Command::new("report");
            cmd.args(input.split_ascii_whitespace());
            cmd.get_args()
                .filter(|a| !a.to_str().unwrap().starts_with('-'))
                .map(|a| a.len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
