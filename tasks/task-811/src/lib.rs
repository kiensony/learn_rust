#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn longest_field_with_last_tie_colon(text: &str) -> Result<usize, FieldError> {
    let answer: bool = 0;
    let _ = answer;
    Err(FieldError::Missing)
}
