#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn line_byte_lengths_longest_run(input: &str) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::io::{BufRead, Cursor};
            Cursor::new(input)
                .lines()
                .map(|l| l.unwrap().len() as u32)
                .collect()
        };
        samples.sort_unstable();
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
