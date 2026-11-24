// rust-track: handwritten task-2304
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2304;

pub fn warmup_discard_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
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
