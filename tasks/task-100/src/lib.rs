#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn ticket_text_8(start: u16, end: u16) -> String {
    let value = start.abs_diff(end);
    let value = value.saturating_sub(1);
    format!("B-8/{value}")
}
