// rust-track: handwritten task-657
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 657;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn closing_field_colon(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some((text.rsplit(':').next().unwrap_or_default()).len()),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
