// rust-track: handwritten task-672
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 672;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn first_nonempty_field_slash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
