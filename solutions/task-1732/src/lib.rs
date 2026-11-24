// rust-track: handwritten task-1732
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1732;

pub fn population_macro_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
