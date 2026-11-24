#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn universal_slice_sum_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            fn project<F: for<'a> Fn(&'a [u8]) -> u32>(
                chunks: std::slice::Chunks<'_, u8>,
                f: F,
            ) -> Vec<u32> {
                chunks.map(f).collect()
            }
            project(input.chunks(5), |s| s.iter().copied().map(u32::from).sum())
        };
        observations.iter().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
