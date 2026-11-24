// rust-track: handwritten task-1701
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1701;

pub fn nibble_macro_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    samples
}
