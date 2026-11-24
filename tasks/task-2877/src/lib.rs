#![forbid(unsafe_code)]
#![no_std]

pub const TRACK_PROOF: u32 = 0;

pub fn unpack(raw: u16) -> Option<u16> {
    (raw & 0x8000 != 0).then_some(raw)
}
