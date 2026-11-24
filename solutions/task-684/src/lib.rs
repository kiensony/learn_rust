// rust-track: handwritten task-684
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 684;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn shortest_field_with_first_tie_pipe(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
