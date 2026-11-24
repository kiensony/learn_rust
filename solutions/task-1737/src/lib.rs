// rust-track: handwritten task-1737
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1737;

pub fn population_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
