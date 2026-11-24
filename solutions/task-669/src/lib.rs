// rust-track: handwritten task-669
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 669;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn marks_removed_at_both_ends_slash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.trim_matches('/')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
