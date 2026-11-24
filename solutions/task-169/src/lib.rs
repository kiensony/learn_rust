// rust-track: handwritten task-169
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 169;

pub fn status_lamp_2(value: u16) -> &'static str {
    if value < 2 {
        "low"
    } else if value > 4 {
        "full"
    } else {
        "ready"
    }
}
