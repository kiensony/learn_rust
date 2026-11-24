// rust-track: handwritten task-834
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 834;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_field_equals(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit('=').next().unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 6 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
