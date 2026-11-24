// rust-track: handwritten task-2214
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2214;

pub fn borrowed_offsets_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
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
