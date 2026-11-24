// rust-track: handwritten task-805
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 805;

#[derive(Debug, PartialEq)]
pub struct BadCount;
pub fn parse_count(text: &str) -> Result<u16, BadCount> {
    let count = text.parse::<u16>().map_err(|_| BadCount)?;
    Ok(count)
}
