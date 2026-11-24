// rust-track: handwritten task-1315
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1315;

pub fn nibble_fields_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples.dedup();
    samples
}
