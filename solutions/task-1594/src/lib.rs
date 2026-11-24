// rust-track: handwritten task-1594
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1594;

pub fn feature_pages_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_feature_pages(input);
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

fn observe_feature_pages(input: &[u8]) -> Vec<u32> {
    let stride = if cfg!(feature = "wide") { 32u32 } else { 1u32 };
    input.iter().map(|&b| u32::from(b) * stride).collect()
}
