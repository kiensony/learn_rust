// rust-track: handwritten task-1312
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1312;

pub fn nibble_fields_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
