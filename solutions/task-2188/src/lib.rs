// rust-track: handwritten task-2188
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2188;

pub fn early_exit_probes_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
