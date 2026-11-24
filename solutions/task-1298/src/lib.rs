// rust-track: handwritten task-1298
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1298;

pub fn explicit_levels_rolling_load(input: &str, width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter_map(|a| a.to_str()?.strip_prefix("--level=")?.parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
