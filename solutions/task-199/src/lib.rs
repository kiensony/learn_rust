// rust-track: handwritten task-199
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 199;

pub fn status_lamp_8(start: u16, end: u16) -> &'static str {
    let value = start.abs_diff(end);
    if value < 8 {
        "low"
    } else if value > 10 {
        "full"
    } else {
        "ready"
    }
}
