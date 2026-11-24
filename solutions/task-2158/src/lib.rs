// rust-track: handwritten task-2158
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2158;

pub fn linear_search_probes_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|needle| input.iter().position(|x| x == needle).unwrap() as u32 + 1)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
