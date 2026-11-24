// rust-track: handwritten task-851
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 851;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn longest_field_with_last_tie_hash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 7 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
