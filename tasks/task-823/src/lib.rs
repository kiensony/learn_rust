#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn leading_field_pipe(text: &str) -> Result<usize, FieldError> {
    let answer: bool = 0;
    let _ = answer;
    Err(FieldError::Missing)
}
