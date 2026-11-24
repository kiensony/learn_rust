// rust-track: handwritten task-2694
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2694;

pub fn universal_slice_width_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_universal_slice_width(input);
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

fn observe_universal_slice_width(input: &[u8]) -> Vec<u32> {
    fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
        chunks: std::slice::Chunks<'_, u8>,
        f: F,
    ) -> Vec<u32> {
        chunks.map(f).collect()
    }
    project(input.chunks(6), |s| {
        u32::from(s.iter().max().unwrap() - s.iter().min().unwrap())
    })
}
