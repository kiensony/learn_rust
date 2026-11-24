// rust-track: handwritten task-151
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 151;

pub fn whole_cartons_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value).div_ceil(10)
}
