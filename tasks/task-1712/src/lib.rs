#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturating_macro_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x.wrapping_add(1))).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
