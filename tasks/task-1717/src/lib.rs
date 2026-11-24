#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturating_macro_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            macro_rules! sample {
                ($x:expr) => {
                    u32::from(($x).saturating_add(4))
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
