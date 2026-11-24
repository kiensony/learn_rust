// rust-track: handwritten task-062
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 62;

pub fn open_sign(is_open: bool) -> bool {
    let visible = is_open;
    visible
}
