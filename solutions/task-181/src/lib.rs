// rust-track: handwritten task-181
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 181;

pub fn last_ticket_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value < 5
}
