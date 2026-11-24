// rust-track: handwritten task-1302
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1302;

pub fn decimal_fields_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
