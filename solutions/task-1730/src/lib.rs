// rust-track: handwritten task-1730
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1730;

pub fn boolean_macro_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x) == 0)
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().rev().collect()
}
