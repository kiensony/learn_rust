// rust-track: handwritten task-2883
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2883;

pub fn packed_pin_state_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
