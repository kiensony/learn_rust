// rust-track: handwritten task-2377
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2377;

pub fn cumulative_occupancy_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut n = 0;
        input
            .iter()
            .map(|&x| {
                n += u32::from(x != 0);
                n
            })
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
