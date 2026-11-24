#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_nibble_swap_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let _ = (input, replay);
    None
}
