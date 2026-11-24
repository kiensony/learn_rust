// rust-track: handwritten task-2695
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2695;

pub fn universal_slice_width_changes(input: &[u8]) -> Vec<u32> {
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
    samples.dedup();
    samples
}
