// rust-track: handwritten task-2934
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2934;

pub fn stability_band_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_stability_band(input);
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

fn observe_stability_band(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(u8::from(p[0].abs_diff(p[1]) <= 2)))
        .collect()
}
