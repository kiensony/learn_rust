// rust-track: handwritten task-1716
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1716;

pub fn saturating_macro_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
