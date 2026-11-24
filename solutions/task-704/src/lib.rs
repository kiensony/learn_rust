// rust-track: handwritten task-704
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 704;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn shortest_field_with_first_tie_hash(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text
                    .split_once('[')
                    .and_then(|(_, tail)| tail.split_once(']'))
                    .map_or("", |(inside, _)| inside);
                text.split('#').min_by_key(|s| s.len()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
