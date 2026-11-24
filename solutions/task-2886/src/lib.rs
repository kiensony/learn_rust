// rust-track: handwritten task-2886
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2886;

pub fn packed_pin_state_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
