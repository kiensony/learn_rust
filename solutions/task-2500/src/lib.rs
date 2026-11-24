// rust-track: handwritten task-2500
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2500;

pub fn interior_raw_reads_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let base = input.as_ptr();
        (1..input.len().saturating_sub(1))
            .map(|i| {
                let ptr = base.wrapping_add(i);
                debug_assert_eq!(ptr, &input[i] as *const u8);
                u32::from(input[i])
            })
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
