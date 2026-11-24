// rust-track: handwritten task-2156
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2156;

pub fn linear_search_probes_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
