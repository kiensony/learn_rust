#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

mod detail {
    pub enum Mode {
        Listen,
        Talk,
        Reserved(u8),
    }
}
pub use detail::Mode;

pub fn wire_value(command: Mode) -> u8 {
    0
}
