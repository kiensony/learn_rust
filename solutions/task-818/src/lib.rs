// rust-track: handwritten task-818
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 818;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn remainder_after_first_mark_slash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 4 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
