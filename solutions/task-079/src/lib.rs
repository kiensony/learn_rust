// rust-track: handwritten task-079
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 79;

pub fn a_full_tray_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value % 3 == 0
}
