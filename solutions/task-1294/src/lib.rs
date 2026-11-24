// rust-track: handwritten task-1294
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1294;

pub fn quiet_option_positions_peaks(input: &str) -> Vec<u32> {
    let samples = observe_quiet_option_positions(input);
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

fn observe_quiet_option_positions(input: &str) -> Vec<u32> {
    let mut cmd = std::process::Command::new("report");
    cmd.args(input.split_ascii_whitespace());
    cmd.get_args()
        .enumerate()
        .filter(|(_, a)| *a == "-q")
        .map(|(i, _)| i as u32)
        .collect()
}
