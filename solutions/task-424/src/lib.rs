// rust-track: handwritten task-424
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 424;

pub fn prefix_before_final_mark_slash(text: &mut String) -> () {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
