// rust-track: handwritten task-676
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 676;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn closing_field_pipe(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.rsplit('|').next().unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
