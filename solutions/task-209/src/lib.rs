// rust-track: handwritten task-209
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 209;

pub fn status_lamp_10(requested: u16, available: u16) -> &'static str {
    let value = requested.min(available);
    if value < 10 {
        "low"
    } else if value > 12 {
        "full"
    } else {
        "ready"
    }
}
