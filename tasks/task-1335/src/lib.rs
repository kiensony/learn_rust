#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn field_indexes_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .enumerate()
            .filter(|(_, x)| *x == 0)
            .map(|(i, _)| i as u32)
            .collect()
    };
    samples.dedup();
    samples
}
