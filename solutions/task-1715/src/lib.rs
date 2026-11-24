// rust-track: handwritten task-1715
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1715;

pub fn saturating_macro_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).saturating_add(4))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.dedup();
    samples
}
