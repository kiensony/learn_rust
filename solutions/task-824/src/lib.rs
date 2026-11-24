// rust-track: handwritten task-824
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 824;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_field_pipe(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 5 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
