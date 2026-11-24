// rust-track: handwritten task-182
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 182;

pub fn comfort_zone_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    (5..=7).contains(&value)
}
