#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cumulative_occupancy_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
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
        samples.reverse();
        samples
    };
    samples.into_iter().rev().collect()
}
