#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn option_terminator_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace().take(2));
        cmd.get_args()
            .skip_while(|a| *a != "--")
            .skip(1)
            .map(|a| a.len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
