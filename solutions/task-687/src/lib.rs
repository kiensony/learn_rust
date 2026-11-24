// rust-track: handwritten task-687
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 687;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn leading_marks_removed_equals(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.split_once(';').map_or(text, |(data, _)| data);
                text.trim_start_matches('=')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
