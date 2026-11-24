// rust-track: handwritten task-285
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 285;

pub fn next_room(room: u16) -> u32 {
    u32::from(room) + 1
}
