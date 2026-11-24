// rust-track: handwritten task-435
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 435;

pub fn first_nonempty_field_pipe(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
