// rust-track: handwritten task-2234
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2234;

pub fn zero_copy_lines_peaks(input: &str) -> Vec<u32> {
    let samples = observe_zero_copy_lines(input);
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

fn observe_zero_copy_lines(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.len() as u32).collect()
}
