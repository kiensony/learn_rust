#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn borrowed_windows_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
    let _ = (input, replay);
    None
}
