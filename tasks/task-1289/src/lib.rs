#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn option_terminator_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            let mut cmd = std::process::Command::new("report");
            cmd.args(input.split_ascii_whitespace());
            cmd.get_args()
                .skip_while(|a| *a != "--")
                .skip(1)
                .map(|a| a.len() as u32)
                .collect()
        };
        observations.iter().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
