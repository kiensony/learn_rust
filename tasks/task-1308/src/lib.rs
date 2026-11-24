#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn decimal_fields_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(u32::from)
            .collect()
    };
    let _ = samples;
    Vec::new()
}
