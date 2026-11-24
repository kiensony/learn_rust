#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn marks_removed_at_both_ends_colon(text: &str) -> Result<usize, FieldError> {
    Err(FieldError::Missing)
}
