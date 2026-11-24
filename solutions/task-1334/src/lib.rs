// rust-track: handwritten task-1334
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1334;

pub fn field_indexes_peaks(input: &str) -> Vec<u32> {
    let samples = observe_field_indexes(input);
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

fn observe_field_indexes(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .enumerate()
        .filter(|(_, x)| *x == 0)
        .map(|(i, _)| i as u32)
        .collect()
}
