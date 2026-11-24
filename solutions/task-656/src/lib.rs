// rust-track: handwritten task-656
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 656;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn leading_field_colon(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some((text.split(':').next().unwrap_or_default()).len()),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
