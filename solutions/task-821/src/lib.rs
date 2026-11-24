// rust-track: handwritten task-821
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 821;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn longest_field_with_last_tie_slash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 4 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
