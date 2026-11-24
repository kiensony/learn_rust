#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn rotate_macro_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                u32::from(($x).rotate_left(2))
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    let _ = samples;
    Vec::new()
}
