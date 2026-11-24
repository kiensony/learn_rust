// rust-track: handwritten task-660
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 660;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn marks_removed_at_both_ends_colon(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some((text.trim_matches(':')).len()),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
