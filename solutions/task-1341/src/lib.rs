// rust-track: handwritten task-1341
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1341;

pub fn packed_booleans_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| x.count_ones())
            .collect()
    };
    samples
}
