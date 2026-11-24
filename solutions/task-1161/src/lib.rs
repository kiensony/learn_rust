// rust-track: handwritten task-1161
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1161;

pub fn setting(line: &str) -> Option<(&str, &str)> {
    line.split_once('=')
}
