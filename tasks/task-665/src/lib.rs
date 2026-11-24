#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Signal {
    Go,
    Wait(u16),
    Closed,
}

pub fn wait_seconds(signal: Signal) -> u16 {
    0
}
