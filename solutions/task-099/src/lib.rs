// rust-track: handwritten task-099
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 99;

pub fn a_full_tray_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value % 8 == 0
}
