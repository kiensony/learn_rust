// rust-track: handwritten task-659
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 659;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn closing_marks_removed_colon(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some((text.trim_end_matches(':')).len()),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
