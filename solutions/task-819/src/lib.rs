// rust-track: handwritten task-819
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 819;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn prefix_before_final_mark_slash(text: &str) -> Result<usize, FieldError> {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(text: &str) -> Result<usize, FieldError> {
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    };
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 4 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
