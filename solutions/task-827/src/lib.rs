// rust-track: handwritten task-827
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 827;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn marks_removed_at_both_ends_pipe(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 5 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
