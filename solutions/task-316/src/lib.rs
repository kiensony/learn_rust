// rust-track: handwritten task-316
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 316;

pub fn release(ticket: String) -> usize {
    let mut ticket = ticket.into_bytes();
    let count = ticket.len();
    drop(ticket);
    count
}
