#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn universal_slice_sum_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
                chunks: std::slice::Chunks<'_, u8>,
                f: F,
            ) -> Vec<u32> {
                chunks.map(f).collect()
            }
            project(input.chunks(5), |s| s.iter().copied().map(u32::from).sum())
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
