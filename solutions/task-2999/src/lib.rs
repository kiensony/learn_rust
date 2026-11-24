// rust-track: handwritten task-2999
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2999;

pub fn handoff(input: Vec<u8>, capacity: usize) -> (Vec<u8>, Vec<u8>) {
    let mut accepted = input;
    let cut = capacity.min(accepted.len());
    let pending = accepted.split_off(cut);
    (accepted, pending)
}
