// rust-track: handwritten task-665
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 665;

pub enum Signal {
    Go,
    Wait(u16),
    Closed,
}

pub fn wait_seconds(signal: Signal) -> u16 {
    match signal {
        Signal::Go => 0,
        Signal::Wait(seconds) => seconds,
        Signal::Closed => u16::MAX,
    }
}
