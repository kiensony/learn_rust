// rust-track: handwritten task-1604
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1604;

pub fn parity_property_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
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
