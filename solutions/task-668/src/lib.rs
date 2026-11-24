// rust-track: handwritten task-668
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 668;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn closing_marks_removed_slash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.trim_end_matches('/')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
