// rust-track: handwritten task-835
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 835;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn leading_marks_removed_equals(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 6 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
