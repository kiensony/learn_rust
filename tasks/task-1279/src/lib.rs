#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn explicit_levels_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut cmd = std::process::Command::new("report");
            cmd.args(input.split_ascii_whitespace());
            cmd.get_args()
                .filter_map(|a| a.to_str()?.strip_prefix("--level=")?.parse::<u8>().ok())
                .map(u32::from)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
