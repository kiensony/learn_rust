// rust-track: handwritten task-2376
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2376;

pub fn cumulative_occupancy_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
