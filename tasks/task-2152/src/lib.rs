#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn linear_search_probes_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    let _ = samples;
    0
}
