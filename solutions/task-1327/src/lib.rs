// rust-track: handwritten task-1327
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1327;

pub fn flag_fields_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
