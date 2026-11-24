#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn shortest_field_with_first_tie_colon(text: &str) -> Result<usize, FieldError> {
    Err(FieldError::Missing)
}
