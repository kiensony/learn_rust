#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn delimited_fields_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(255)
            .map(|s| s.unwrap().len() as u32)
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
