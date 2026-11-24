// rust-track: handwritten task-814
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 814;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_field_slash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 4 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
