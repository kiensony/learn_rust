#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_macro_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let _ = samples;
    0
}
