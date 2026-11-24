// rust-track: handwritten task-1287
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1287;

pub fn option_terminator_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .skip_while(|a| *a != "--")
            .skip(1)
            .map(|a| a.len() as u32)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
