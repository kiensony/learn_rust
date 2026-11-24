#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packed_pin_state_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .windows(2)
            .map(|c| u32::from(c[0].rotate_left(u32::from(c[1] % 8))))
            .collect()
    };
    samples.into_iter().rev().collect()
}
