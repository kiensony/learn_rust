// rust-track: handwritten task-1720
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1720;

pub fn saturating_macro_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().rev().collect()
}
