#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum Field {
    Left,
    Right,
}

pub fn combine(left: Result<u8, Field>, right: Result<u8, Field>) -> Result<u16, Field> {
    right.and_then(|r| left.map(|l| u16::from(l) + u16::from(r)))
}
