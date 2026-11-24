// rust-track: handwritten task-090
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 90;

pub fn ticket_text_5(issued: u16, returned: u16) -> String {
    let value = issued.saturating_sub(returned);
    format!("B-5/{value}")
}
