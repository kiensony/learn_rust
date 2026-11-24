// rust-track: handwritten task-2450
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2450;

pub fn page_alignment_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        #[repr(align(32))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
