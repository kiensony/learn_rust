#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_variation_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks(4)
                .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
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
