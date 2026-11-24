// rust-track: handwritten task-1272
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1272;

pub fn explicit_levels_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter_map(|a| a.to_str()?.strip_prefix("--level=")?.parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
