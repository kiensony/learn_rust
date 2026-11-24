// rust-track: handwritten task-119
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 119;

pub fn offset(extra: u8) -> u8 {
    200u8.saturating_add(extra)
}
