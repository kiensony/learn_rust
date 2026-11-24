// rust-track: handwritten task-1735
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1735;

pub fn population_macro_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x).count_ones()
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.dedup();
    samples
}
