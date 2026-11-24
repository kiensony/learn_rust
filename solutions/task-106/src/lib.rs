// rust-track: handwritten task-106
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 106;

pub fn bundle_labels_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value) * (10 + 1)
}
