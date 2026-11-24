// rust-track: handwritten task-1316
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1316;

pub fn nibble_fields_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
