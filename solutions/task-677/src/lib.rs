// rust-track: handwritten task-677
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 677;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn leading_marks_removed_pipe(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.trim_start_matches('|')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
