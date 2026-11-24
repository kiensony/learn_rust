// rust-track: handwritten task-2373
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2373;

pub fn cumulative_occupancy_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
