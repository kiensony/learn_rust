#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn population_macro_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let _ = samples;
    Vec::new()
}
