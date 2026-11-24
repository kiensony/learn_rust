// rust-track: handwritten task-100
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 100;

pub fn ticket_text_8(start: u16, end: u16) -> String {
    let value = start.abs_diff(end);
    format!("B-8/{value}")
}
