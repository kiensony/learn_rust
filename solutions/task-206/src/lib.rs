// rust-track: handwritten task-206
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 206;

pub fn admitted(member: bool, invited: bool, closed: bool) -> bool {
    !closed && (member || invited)
}
