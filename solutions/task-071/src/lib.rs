// rust-track: handwritten task-071
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 71;

pub fn ticket_mark(mark: char) -> (char, bool) {
    (mark, mark == '*')
}
