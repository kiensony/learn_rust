// rust-track: handwritten task-096
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 96;

pub fn bundle_labels_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value) * (8 + 1)
}
