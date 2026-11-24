// rust-track: handwritten task-201
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 201;

pub fn last_ticket_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value < 10
}
