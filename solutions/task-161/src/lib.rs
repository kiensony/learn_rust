// rust-track: handwritten task-161
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 161;

pub fn door(locked: bool) -> &'static str {
    if locked { "locked" } else { "open" }
}
