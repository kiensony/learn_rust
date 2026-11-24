// rust-track: handwritten task-110
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 110;

pub fn ticket_text_10(requested: u16, available: u16) -> String {
    let value = requested.min(available);
    format!("B-10/{value}")
}
