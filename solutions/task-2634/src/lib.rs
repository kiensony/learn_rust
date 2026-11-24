// rust-track: handwritten task-2634
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2634;

pub fn embedded_saturation_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_embedded_saturation(input);
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

fn observe_embedded_saturation(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(track_device::convert(x)))
        .collect()
}
