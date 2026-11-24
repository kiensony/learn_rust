#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn prefix_before_final_mark_equals(text: &str) -> Result<usize, FieldError> {
    Err(FieldError::Missing)
}
