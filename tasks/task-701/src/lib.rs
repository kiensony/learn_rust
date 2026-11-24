#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Motion {
    Home,
    Shift(i16),
    Stay,
}

pub fn position(command: Motion, at: i16) -> i16 {
    at
}
