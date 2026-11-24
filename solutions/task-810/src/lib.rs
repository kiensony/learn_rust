// rust-track: handwritten task-810
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 810;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn first_nonempty_field_colon(text: &str) -> Result<usize, FieldError> {
    let field = text.split(':').find(|s| !s.is_empty()).unwrap_or_default();
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
