// rust-track: handwritten task-846
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 846;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn closing_marks_removed_hash(text: &str) -> Result<usize, FieldError> {
    closing_marks_removed_hash_impl(text)
}

fn closing_marks_removed_hash_impl(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 7 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
