// rust-track: handwritten task-1154
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1154;

pub fn closing_marks_removed_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.trim_end_matches(':');
    if found.is_empty() { fallback } else { found }
}
