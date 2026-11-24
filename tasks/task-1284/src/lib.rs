#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn option_terminator_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut cmd = std::process::Command::new("report");
            cmd.args(input.split_ascii_whitespace());
            cmd.get_args()
                .skip_while(|a| *a != "--")
                .skip(1)
                .map(|a| a.len() as u32)
                .collect()
        };
        samples.reverse();
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
