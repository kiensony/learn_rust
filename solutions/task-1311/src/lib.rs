// rust-track: handwritten task-1311
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1311;

pub fn nibble_fields_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples
}
