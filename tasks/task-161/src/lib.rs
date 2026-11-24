#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn door(locked: bool) -> &'static str {
    "open"
}
