// rust-track: handwritten task-437
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 437;

pub fn shortest_field_with_first_tie_pipe(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
