// rust-track: handwritten task-1950
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1950;

pub fn locked_batch_peaks_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let journal = std::sync::Mutex::new(Vec::new());
        for s in input.chunks(3) {
            let item = u32::from(*s.iter().max().unwrap());
            journal.lock().unwrap().push(item);
        }
        journal.into_inner().unwrap()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
