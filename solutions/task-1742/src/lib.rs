// rust-track: handwritten task-1742
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1742;

pub fn rotate_macro_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).rotate_left(2))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
