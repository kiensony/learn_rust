// rust-track: handwritten task-080
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 80;

pub fn ticket_text_3(first: u16, second: u16) -> String {
    let value = first.saturating_add(second);
    format!("B-3/{value}")
}
