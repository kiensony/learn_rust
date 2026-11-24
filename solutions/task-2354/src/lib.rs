// rust-track: handwritten task-2354
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2354;

pub fn single_pass_prefix_maximum_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut m = 0;
        input
            .iter()
            .map(|&x| {
                m = m.max(x);
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
