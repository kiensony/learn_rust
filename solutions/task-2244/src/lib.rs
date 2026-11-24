// rust-track: handwritten task-2244
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2244;

pub fn borrowed_windows_peaks(input: &str) -> Vec<u32> {
    let samples = observe_borrowed_windows(input);
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

fn observe_borrowed_windows(input: &str) -> Vec<u32> {
    input
        .as_bytes()
        .windows(2)
        .map(|w| u32::from(u16::from_be_bytes([w[0], w[1]])))
        .collect()
}
