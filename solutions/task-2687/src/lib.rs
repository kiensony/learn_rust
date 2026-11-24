// rust-track: handwritten task-2687
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2687;

pub fn universal_slice_sum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
            chunks: std::slice::Chunks<'_, u8>,
            f: F,
        ) -> Vec<u32> {
            chunks.map(f).collect()
        }
        project(input.chunks(5), |s| s.iter().copied().map(u32::from).sum())
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
