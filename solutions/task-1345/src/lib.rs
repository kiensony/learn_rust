// rust-track: handwritten task-1345
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1345;

pub fn packed_booleans_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| x.count_ones())
            .collect()
    };
    samples.dedup();
    samples
}
