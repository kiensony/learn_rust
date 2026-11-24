// rust-track: handwritten task-847
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 847;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn marks_removed_at_both_ends_hash(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_matches('#')
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 7 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
