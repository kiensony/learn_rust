// rust-track: handwritten task-699
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 699;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn marks_removed_at_both_ends_hash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.trim_matches('#')
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
