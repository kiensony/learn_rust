// rust-track: handwritten task-2877
#![forbid(unsafe_code)]
#![no_std]

pub const TRACK_PROOF: u32 = 0x52540000 | 2877;

pub fn unpack(raw: u16) -> Option<u16> {
    (raw & 0x8000 != 0).then_some(raw & 0x0fff)
}
