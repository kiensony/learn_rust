// rust-track: handwritten task-1723
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1723;

pub fn boolean_macro_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x) == 0)
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
