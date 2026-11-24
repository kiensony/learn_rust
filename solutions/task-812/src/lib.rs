// rust-track: handwritten task-812
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 812;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn shortest_field_with_first_tie_colon(text: &str) -> Result<usize, FieldError> {
    let field = text.split(':').min_by_key(|s| s.len()).unwrap_or_default();
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
