// rust-track: handwritten task-809
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 809;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldError {
    Missing,
    TooLong,
}

pub fn prefix_before_final_mark_colon(text: &str) -> Result<usize, FieldError> {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl(text: &str) -> Result<usize, FieldError> {
    let field = text.rsplit_once(':').map_or("", |(a, _)| a);
    if field.is_empty() {
        Err(FieldError::Missing)
    } else if field.len() > 3 {
        Err(FieldError::TooLong)
    } else {
        Ok(field.len())
    }
}
