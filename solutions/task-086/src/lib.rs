// rust-track: handwritten task-086
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 86;

pub fn bundle_labels_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value) * (5 + 1)
}
