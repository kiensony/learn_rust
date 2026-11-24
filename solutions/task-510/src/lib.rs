// rust-track: handwritten task-510
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 510;

pub fn is_tab(ch: char) -> bool {
    ch == '\t'
}
