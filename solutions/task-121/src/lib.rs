// rust-track: handwritten task-121
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 121;

pub fn as_byte(reading: i16) -> Option<u8> {
    u8::try_from(reading).ok()
}
