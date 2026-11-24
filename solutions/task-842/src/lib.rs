// rust-track: handwritten task-842
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 842;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn shortest_field_with_first_tie_equals(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 6 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
