// rust-track: handwritten task-189
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 189;

pub fn status_lamp_5(issued: u16, returned: u16) -> &'static str {
    let value = issued.saturating_sub(returned);
    if value < 5 {
        "low"
    } else if value > 7 {
        "full"
    } else {
        "ready"
    }
}
