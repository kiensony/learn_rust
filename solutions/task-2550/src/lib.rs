// rust-track: handwritten task-2550
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2550;

pub fn abi_status_mask_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x & 0x7f)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
