// rust-track: handwritten task-2381
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2381;

pub fn rolling_parity_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut p = 0;
        input
            .iter()
            .map(|&x| {
                p ^= x;
                u32::from(p)
            })
            .collect()
    };
    samples
}
