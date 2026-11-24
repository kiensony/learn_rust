// rust-track: handwritten task-2186
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2186;

pub fn early_exit_probes_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
