// rust-track: handwritten task-1330
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1330;

pub fn flag_fields_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
            .collect()
    };
    samples.into_iter().rev().collect()
}
