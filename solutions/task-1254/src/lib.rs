// rust-track: handwritten task-1254
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1254;

pub fn command_arguments_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .map(|a| a.to_str().unwrap().len() as u32)
            .collect()
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
