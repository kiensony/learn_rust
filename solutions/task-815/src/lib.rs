// rust-track: handwritten task-815
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 815;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn leading_marks_removed_slash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 4 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
