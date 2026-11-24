// rust-track: handwritten task-1708
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1708;

pub fn nibble_macro_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
