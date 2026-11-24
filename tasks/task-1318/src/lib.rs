#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nibble_fields_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .skip(1)
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| u32::from(x >> 4))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
