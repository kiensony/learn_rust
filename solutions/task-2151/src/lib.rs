// rust-track: handwritten task-2151
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2151;

pub fn linear_search_probes_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    samples
}
