// rust-track: handwritten task-1284
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1284;

pub fn option_terminator_peaks(input: &str) -> Vec<u32> {
    let samples = observe_option_terminator(input);
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

fn observe_option_terminator(input: &str) -> Vec<u32> {
    let mut cmd = std::process::Command::new("report");
    cmd.args(input.split_ascii_whitespace());
    cmd.get_args()
        .skip_while(|a| *a != "--")
        .skip(1)
        .map(|a| a.len() as u32)
        .collect()
}
