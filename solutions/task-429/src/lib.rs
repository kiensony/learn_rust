// rust-track: handwritten task-429
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 429;

pub fn closing_field_pipe(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
