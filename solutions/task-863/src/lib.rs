// rust-track: handwritten task-863
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 863;

mod protocol {
    pub enum Command {
        Exit,
        Read,
        Write(u8),
    }
}
pub use protocol::Command;

pub fn is_exit(command: Command) -> bool {
    matches!(command, Command::Exit)
}
