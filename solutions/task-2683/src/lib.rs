// rust-track: handwritten task-2683
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2683;

pub fn universal_slice_sum_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
            chunks: std::slice::Chunks<'_, u8>,
            f: F,
        ) -> Vec<u32> {
            chunks.map(f).collect()
        }
        project(input.chunks(5), |s| s.iter().copied().map(u32::from).sum())
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
