// rust-track: handwritten task-836
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 836;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_marks_removed_equals(text: &str) -> Result<usize, FieldError> {
    closing_marks_removed_equals_impl(text)
}

fn closing_marks_removed_equals_impl(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 6 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
