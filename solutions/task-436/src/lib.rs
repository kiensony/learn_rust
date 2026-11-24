// rust-track: handwritten task-436
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 436;

pub fn longest_field_with_last_tie_pipe(text: &mut String) -> () {
    longest_field_with_last_tie_pipe_impl(text)
}

fn longest_field_with_last_tie_pipe_impl(text: &mut String) -> () {
    let selected = ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .to_owned();
    text.clear();
    text.push_str(&selected);
}
