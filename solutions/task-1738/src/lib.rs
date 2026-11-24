// rust-track: handwritten task-1738
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1738;

pub fn population_macro_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
