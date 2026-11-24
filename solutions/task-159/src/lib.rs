// rust-track: handwritten task-159
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 159;

pub fn nearest_group_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    ((u32::from(value) + 5) / 10) * 10
}
