// rust-track: handwritten task-076
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 76;

pub fn bundle_labels_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) * (3 + 1)
}
