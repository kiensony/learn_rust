#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn explicit_levels_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace().take(2));
        cmd.get_args()
            .filter_map(|a| a.to_str()?.strip_prefix("--level=")?.parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
