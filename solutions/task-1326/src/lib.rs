// rust-track: handwritten task-1326
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1326;

pub fn flag_fields_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
