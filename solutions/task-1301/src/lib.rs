// rust-track: handwritten task-1301
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1301;

pub fn decimal_fields_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples
}
