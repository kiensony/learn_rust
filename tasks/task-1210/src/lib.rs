#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn line_byte_lengths_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples.into_iter().rev().collect()
}
