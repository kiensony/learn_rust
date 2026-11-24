// rust-track: handwritten task-692
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 692;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn first_nonempty_field_equals(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.split_once(';').map_or(text, |(data, _)| data);
                text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
