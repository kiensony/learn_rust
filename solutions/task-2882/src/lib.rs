// rust-track: handwritten task-2882
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2882;

pub fn packed_pin_state_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
