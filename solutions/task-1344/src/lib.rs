// rust-track: handwritten task-1344
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1344;

pub fn packed_booleans_peaks(input: &str) -> Vec<u32> {
    let samples = observe_packed_booleans(input);
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

fn observe_packed_booleans(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .map(|x| x.count_ones())
        .collect()
}
