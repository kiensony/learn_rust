// rust-track: handwritten task-2372
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2372;

pub fn cumulative_occupancy_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
