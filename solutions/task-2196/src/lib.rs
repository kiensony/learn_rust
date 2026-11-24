// rust-track: handwritten task-2196
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2196;

pub fn linear_search_probes_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
