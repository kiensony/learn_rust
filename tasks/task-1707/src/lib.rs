#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            macro_rules! sample {
                ($x:expr) => {
                    ($x & 15) as u32
                };
            }
            input.iter().map(|&x| sample!(x)).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
