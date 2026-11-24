// rust-track: handwritten task-171
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 171;

pub fn last_ticket_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value < 3
}
