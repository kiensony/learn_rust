// rust-track: handwritten task-1305
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1305;

pub fn decimal_fields_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.dedup();
    samples
}
