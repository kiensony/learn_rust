// rust-track: handwritten task-1263
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1263;

pub fn positional_options_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter(|a| !a.to_str().unwrap().starts_with('-'))
            .map(|a| a.len() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
