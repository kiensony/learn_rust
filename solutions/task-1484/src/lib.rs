// rust-track: handwritten task-1484
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1484;

pub fn response_boundaries_peaks(input: &str) -> Vec<u32> {
    let samples = observe_response_boundaries(input);
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

fn observe_response_boundaries(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .filter(|s| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit()))
        .filter_map(|s| s.parse::<u32>().ok())
        .filter(|x| (100..600).contains(x))
        .enumerate()
        .filter(|(_, x)| *x < 400)
        .map(|(i, _)| i as u32)
        .collect()
}
