// rust-track: handwritten task-2153
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2153;

pub fn linear_search_probes_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
