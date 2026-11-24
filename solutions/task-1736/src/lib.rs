// rust-track: handwritten task-1736
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1736;

pub fn population_macro_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
