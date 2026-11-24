// rust-track: handwritten task-2187
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2187;

pub fn early_exit_probes_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
