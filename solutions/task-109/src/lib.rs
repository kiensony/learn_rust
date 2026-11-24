// rust-track: handwritten task-109
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 109;

pub fn a_full_tray_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value % 10 == 0
}
