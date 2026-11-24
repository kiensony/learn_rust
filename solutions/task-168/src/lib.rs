// rust-track: handwritten task-168
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 168;

pub fn status(stations: u32) -> &'static str {
    if stations == 0 { "offline" } else { "online" }
}
