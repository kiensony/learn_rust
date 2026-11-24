#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_register_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0] & c[1]))
            .collect()
    };
    let _ = samples;
    0
}
