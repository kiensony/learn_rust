// rust-track: handwritten task-1743
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1743;

pub fn rotate_macro_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).rotate_left(2))
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
