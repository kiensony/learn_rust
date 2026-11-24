// rust-track: handwritten task-807
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 807;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn marks_removed_at_both_ends_colon(text: &str) -> Result<usize, FieldError> {
    let field = text.trim_matches(':');
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
