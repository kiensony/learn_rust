// rust-track: handwritten task-1740
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1740;

pub fn population_macro_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().rev().collect()
}
