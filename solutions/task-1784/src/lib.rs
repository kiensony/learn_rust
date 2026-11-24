// rust-track: handwritten task-1784
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1784;

pub fn token_multiplier_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_token_multiplier(input);
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

fn observe_token_multiplier(input: &[u8]) -> Vec<u32> {
    let units: u32 = arity::arity!(unit unit unit unit);
    input.iter().map(|&x| u32::from(x) * units).collect()
}
