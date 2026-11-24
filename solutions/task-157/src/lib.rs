// rust-track: handwritten task-157
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 157;

pub fn milliseconds(seconds: i32) -> Option<u32> {
    u32::try_from(seconds).ok()?.checked_mul(1000)
}
