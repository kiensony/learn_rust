// rust-track: handwritten task-1286
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1286;

pub fn option_terminator_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .skip_while(|a| *a != "--")
            .skip(1)
            .map(|a| a.len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
