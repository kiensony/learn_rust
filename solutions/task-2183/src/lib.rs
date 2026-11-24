// rust-track: handwritten task-2183
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2183;

pub fn early_exit_probes_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
