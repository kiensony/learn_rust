// rust-track: handwritten task-662
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 662;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn first_nonempty_field_colon(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => {
            Some((text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len())
        }
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
