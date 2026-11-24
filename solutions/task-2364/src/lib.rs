// rust-track: handwritten task-2364
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2364;

pub fn single_pass_prefix_minimum_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut m = u8::MAX;
        input
            .iter()
            .map(|&x| {
                m = m.min(x);
                u32::from(m)
            })
            .collect()
    };
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
