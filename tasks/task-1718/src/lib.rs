#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturating_macro_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x.wrapping_add(1))).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
