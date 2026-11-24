#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn marks_removed_at_both_ends_slash(text: &str) -> Result<usize, FieldError> {
    let answer: bool = 0;
    let _ = answer;
    Err(FieldError::Missing)
}
