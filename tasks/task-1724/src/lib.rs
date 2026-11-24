#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn boolean_macro_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x) == 0)
            };
        }
        input.iter().map(|&x| sample!(x.wrapping_add(1))).collect()
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
