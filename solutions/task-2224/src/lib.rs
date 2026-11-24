// rust-track: handwritten task-2224
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2224;

pub fn borrowed_scalars_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
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
