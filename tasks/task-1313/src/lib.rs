#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_fields_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
