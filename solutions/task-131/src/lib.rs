// rust-track: handwritten task-131
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 131;

pub fn whole_cartons_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value).div_ceil(5)
}
