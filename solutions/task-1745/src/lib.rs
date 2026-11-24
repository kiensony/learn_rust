// rust-track: handwritten task-1745
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1745;

pub fn rotate_macro_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).rotate_left(2))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.dedup();
    samples
}
