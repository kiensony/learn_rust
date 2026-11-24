// rust-track: handwritten task-697
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 697;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn leading_marks_removed_hash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.trim_start_matches('#')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
