// rust-track: handwritten task-341
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 341;

pub fn closing_marks_removed_equals(mut text: String) -> String {
    closing_marks_removed_equals_impl(text)
}

fn closing_marks_removed_equals_impl(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
