#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packed_booleans_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| x.count_ones())
            .collect()
    };
    let _ = samples;
    Vec::new()
}
