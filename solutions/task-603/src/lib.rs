// rust-track: handwritten task-603
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 603;

pub fn choose<'a>(urgent: bool, alert: &'a str, normal: &'a str) -> &'a str {
    if urgent { alert } else { normal }
}
