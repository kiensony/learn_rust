#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn zero_copy_lines_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples
}
