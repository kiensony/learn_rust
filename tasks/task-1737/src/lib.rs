#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn population_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x.wrapping_add(1))).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
