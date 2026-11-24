// rust-track: handwritten task-2700
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2700;

pub fn universal_slice_width_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
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
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
