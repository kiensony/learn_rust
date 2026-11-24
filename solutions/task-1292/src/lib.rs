// rust-track: handwritten task-1292
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1292;

pub fn quiet_option_positions_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .enumerate()
            .filter(|(_, a)| *a == "-q")
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
