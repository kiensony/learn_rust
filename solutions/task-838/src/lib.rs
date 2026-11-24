// rust-track: handwritten task-838
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 838;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn remainder_after_first_mark_equals(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 6 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
