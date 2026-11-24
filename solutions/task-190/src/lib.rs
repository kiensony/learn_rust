// rust-track: handwritten task-190
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 190;

pub fn admission_gate_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value >= 8
}
