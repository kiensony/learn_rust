// rust-track: handwritten task-305
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 305;

pub fn needs_update(old: &str, new: &str) -> bool {
    old.trim() != new.trim()
}
