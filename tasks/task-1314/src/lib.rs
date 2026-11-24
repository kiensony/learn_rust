#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_fields_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
