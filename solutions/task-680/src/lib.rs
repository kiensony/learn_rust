// rust-track: handwritten task-680
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 680;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn remainder_after_first_mark_pipe(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split_once('|').map_or("", |(_, b)| b)
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
