// rust-track: handwritten task-324
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 324;

pub fn prefix_before_final_mark_slash(mut text: String) -> String {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
