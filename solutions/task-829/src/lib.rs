// rust-track: handwritten task-829
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 829;

#[derive(Debug, PartialEq, Eq)]
pub enum Field {
    Left,
    Right,
}

pub fn combine(left: Result<u8, Field>, right: Result<u8, Field>) -> Result<u16, Field> {
    Ok(u16::from(left?) + u16::from(right?))
}
