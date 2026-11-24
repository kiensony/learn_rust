#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_register_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.windows(2).map(|c| u32::from(c[0] & c[1])).collect() };
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
