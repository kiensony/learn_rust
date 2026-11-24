// rust-track: handwritten task-2378
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2378;

pub fn cumulative_occupancy_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut n = 0;
        input
            .iter()
            .map(|&x| {
                n += u32::from(x != 0);
                n
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
