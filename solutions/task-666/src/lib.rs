// rust-track: handwritten task-666
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 666;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn closing_field_slash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.rsplit('/').next().unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
