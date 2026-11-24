// rust-track: handwritten task-1271
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1271;

pub fn explicit_levels_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter_map(|a| a.to_str()?.strip_prefix("--level=")?.parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples
}
