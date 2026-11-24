// rust-track: handwritten task-811
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 811;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn longest_field_with_last_tie_colon(text: &str) -> Result<usize, FieldError> {
    let field = text.split(':').max_by_key(|s| s.len()).unwrap_or_default();
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
