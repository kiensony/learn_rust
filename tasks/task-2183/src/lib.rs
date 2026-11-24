#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn early_exit_probes_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .enumerate()
                .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
