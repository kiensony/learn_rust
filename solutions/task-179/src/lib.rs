// rust-track: handwritten task-179
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 179;

pub fn status_lamp_3(first: u16, second: u16) -> &'static str {
    let value = first.saturating_add(second);
    if value < 3 {
        "low"
    } else if value > 5 {
        "full"
    } else {
        "ready"
    }
}
