// rust-track: handwritten task-2887
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2887;

pub fn packed_pin_state_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
