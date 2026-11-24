// rust-track: handwritten task-1012
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1012;

pub fn swap_pair<T>(pair: (T, T)) -> (T, T) {
    let (left, right) = pair;
    (right, left)
}
