// rust-track: handwritten task-2157
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2157;

pub fn linear_search_probes_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_linear_search_probes(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_linear_search_probes(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
        .collect()
}
