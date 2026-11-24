#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn early_exit_probes_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .enumerate()
                .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().rev().collect()
}
