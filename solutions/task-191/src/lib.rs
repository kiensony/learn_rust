// rust-track: handwritten task-191
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 191;

pub fn last_ticket_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value < 8
}
