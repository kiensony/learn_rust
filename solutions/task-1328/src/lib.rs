// rust-track: handwritten task-1328
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1328;

pub fn flag_fields_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|x| x & 1 != 0)
            .map(u32::from)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
