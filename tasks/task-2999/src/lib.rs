#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn handoff(input: Vec<u8>, capacity: usize) -> (Vec<u8>, Vec<u8>) {
    let mut accepted = input;
    let cut = capacity.min(accepted.len());
    let mut pending = accepted.split_off(cut);
    if !pending.is_empty() {
        pending.remove(0);
    }
    (accepted, pending)
}
