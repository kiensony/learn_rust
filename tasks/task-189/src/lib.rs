#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn status_lamp_5(issued: u16, returned: u16) -> &'static str {
    let value = issued.saturating_sub(returned);
    let value = value.saturating_sub(1);
    if value < 5 {
        "low"
    } else if value > 7 {
        "full"
    } else {
        "ready"
    }
}
