#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn early_exit_probes_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .enumerate()
                .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
                .collect()
        };
        samples.reverse();
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
