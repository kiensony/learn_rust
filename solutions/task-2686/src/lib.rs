// rust-track: handwritten task-2686
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2686;

pub fn universal_slice_sum_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
            chunks: std::slice::Chunks<'_, u8>,
            f: F,
        ) -> Vec<u32> {
            chunks.map(f).collect()
        }
        project(input.chunks(5), |s| s.iter().copied().map(u32::from).sum())
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
