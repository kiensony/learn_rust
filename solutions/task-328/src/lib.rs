// rust-track: handwritten task-328
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 328;

pub fn leading_field_pipe(mut text: String) -> String {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
    text
}
