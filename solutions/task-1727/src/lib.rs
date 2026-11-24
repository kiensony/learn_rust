// rust-track: handwritten task-1727
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1727;

pub fn boolean_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x) == 0)
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
