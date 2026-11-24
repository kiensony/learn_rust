// rust-track: handwritten task-2152
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2152;

pub fn linear_search_probes_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
