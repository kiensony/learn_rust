// rust-track: handwritten task-202
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 202;

pub fn comfort_zone_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    (10..=12).contains(&value)
}
