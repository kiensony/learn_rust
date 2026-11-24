// rust-track: handwritten task-1204
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1204;

pub fn line_byte_lengths_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
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
