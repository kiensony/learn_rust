// rust-track: handwritten task-831
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 831;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn longest_field_with_last_tie_pipe(text: &str) -> Result<usize, FieldError> {
    longest_field_with_last_tie_pipe_impl(text)
}

fn longest_field_with_last_tie_pipe_impl(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 5 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
