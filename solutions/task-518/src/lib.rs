// rust-track: handwritten task-518
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 518;

pub fn scalar_at(text: &str, position: usize) -> Option<char> {
    text.chars().nth(position)
}
