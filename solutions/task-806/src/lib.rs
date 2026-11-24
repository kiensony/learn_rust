// rust-track: handwritten task-806
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 806;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_marks_removed_colon(text: &str) -> Result<usize, FieldError> {
    let field = text.trim_end_matches(':');
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
