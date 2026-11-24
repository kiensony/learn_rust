// rust-track: handwritten task-426
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 426;

pub fn longest_field_with_last_tie_slash(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
