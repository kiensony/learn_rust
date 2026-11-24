#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_fields_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
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
