// rust-track: handwritten task-185
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 185;

pub fn service_interval_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value > 0 && value % 5 == 0
}
