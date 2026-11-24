// rust-track: handwritten task-899
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 899;

mod detail {
    pub enum Mode {
        Listen,
        Talk,
        Reserved(u8),
    }
}
pub use detail::Mode;

pub fn wire_value(command: Mode) -> u8 {
    match command {
        Mode::Listen => 1,
        Mode::Talk => 2,
        Mode::Reserved(n) => n,
    }
}
