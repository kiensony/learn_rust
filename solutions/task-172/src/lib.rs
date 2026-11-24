// rust-track: handwritten task-172
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 172;

pub fn comfort_zone_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    (3..=5).contains(&value)
}
