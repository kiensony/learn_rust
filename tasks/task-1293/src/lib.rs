#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn quiet_option_positions_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace().take(2));
        cmd.get_args()
            .enumerate()
            .filter(|(_, a)| *a == "-q")
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
