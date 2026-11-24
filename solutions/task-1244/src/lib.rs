// rust-track: handwritten task-1244
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1244;

pub fn limited_reads_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_limited_reads(input);
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

fn observe_limited_reads(input: &[u8]) -> Vec<u32> {
    use std::io::{Cursor, Read};
    let mut prefix = Vec::new();
    Cursor::new(input).take(4).read_to_end(&mut prefix).unwrap();
    prefix.into_iter().map(u32::from).collect()
}
