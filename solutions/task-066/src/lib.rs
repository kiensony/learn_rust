// rust-track: handwritten task-066
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 66;

pub fn bundle_labels_2(value: u16) -> u32 {
    u32::from(value) * (2 + 1)
}
