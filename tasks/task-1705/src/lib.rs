#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_macro_changes(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let _ = samples;
    Vec::new()
}
