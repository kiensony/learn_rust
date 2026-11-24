// rust-track: handwritten task-828
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 828;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn remainder_after_first_mark_pipe(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 5 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
