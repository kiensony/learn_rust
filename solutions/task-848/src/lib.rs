// rust-track: handwritten task-848
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 848;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn remainder_after_first_mark_hash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split_once('#').map_or("", |(_, b)| b)
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 7 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
