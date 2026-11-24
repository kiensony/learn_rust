#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_register_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(c[0] & c[1]))
            .collect()
    };
    let _ = samples;
    None
}
