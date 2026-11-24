// rust-track: handwritten task-813
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 813;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadingError {
    Blank,
    Number,
    Range,
}

pub fn reading(text: &str) -> Result<u8, ReadingError> {
    let s = text.trim();
    if s.is_empty() {
        return Err(ReadingError::Blank);
    }
    let n: u16 = s.parse().map_err(|_| ReadingError::Number)?;
    u8::try_from(n).map_err(|_| ReadingError::Range)
}
