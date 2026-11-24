#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn flag_fields_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
            .collect()
    };
    samples
}
