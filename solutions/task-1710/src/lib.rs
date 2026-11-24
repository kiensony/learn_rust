// rust-track: handwritten task-1710
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1710;

pub fn nibble_macro_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().rev().collect()
}
