// rust-track: handwritten task-690
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 690;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn remainder_after_first_mark_equals(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.split_once(';').map_or(text, |(data, _)| data);
                text.split_once('=').map_or("", |(_, b)| b)
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
