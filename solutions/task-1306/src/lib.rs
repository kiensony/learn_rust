// rust-track: handwritten task-1306
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1306;

pub fn decimal_fields_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
