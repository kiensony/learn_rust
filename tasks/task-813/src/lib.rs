#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadingError {
    Blank,
    Number,
    Range,
}

pub fn reading(text: &str) -> Result<u8, ReadingError> {
    Err(ReadingError::Number)
}
