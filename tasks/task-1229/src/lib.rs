#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn delimited_fields_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::io::{BufRead, Cursor};
            Cursor::new(input)
                .split(0)
                .map(|s| s.unwrap().len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
