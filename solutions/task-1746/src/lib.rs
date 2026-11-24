// rust-track: handwritten task-1746
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1746;

pub fn nibble_macro_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        macro_rules! sample {
            ($x:expr) => {
                ($x & 15) as u32
            };
        }
        input.iter().map(|&x| sample!(x)).collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
