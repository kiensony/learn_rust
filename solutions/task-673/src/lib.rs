// rust-track: handwritten task-673
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 673;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn longest_field_with_last_tie_slash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
