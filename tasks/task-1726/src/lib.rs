#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn boolean_macro_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x) == 0)
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let _ = samples;
    0
}
