// rust-track: handwritten task-2334
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2334;

pub fn batch_variation_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_batch_variation(input);
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

fn observe_batch_variation(input: &[u8]) -> Vec<u32> {
    input
        .chunks(4)
        .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
        .collect()
}
