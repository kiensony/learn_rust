#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn ticket_text_5(issued: u16, returned: u16) -> String {
    let value = issued.saturating_sub(returned);
    let value = value.saturating_sub(1);
    format!("B-5/{value}")
}
