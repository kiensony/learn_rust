// rust-track: handwritten task-2182
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2182;

pub fn early_exit_probes_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
