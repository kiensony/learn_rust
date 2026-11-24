// rust-track: handwritten task-1291
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1291;

pub fn quiet_option_positions_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .enumerate()
            .filter(|(_, a)| *a == "-q")
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples
}
