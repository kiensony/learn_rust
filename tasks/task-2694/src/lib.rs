#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn universal_slice_width_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
                chunks: std::slice::Chunks<'_, u8>,
                f: F,
            ) -> Vec<u32> {
                chunks.map(f).collect()
            }
            project(input.chunks(6), |s| {
                u32::from(s.iter().max().unwrap() - s.iter().min().unwrap())
            })
        };
        samples.truncate(samples.len().saturating_sub(1));
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
