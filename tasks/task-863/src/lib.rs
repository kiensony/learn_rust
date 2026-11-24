#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

mod protocol {
    pub enum Command {
        Exit,
        Read,
        Write(u8),
    }
}
pub use protocol::Command;

pub fn is_exit(command: Command) -> bool {
    false
}
