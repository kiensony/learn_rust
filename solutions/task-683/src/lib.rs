// rust-track: handwritten task-683
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 683;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn longest_field_with_last_tie_pipe(event: Event<'_>) -> Option<usize> {
    longest_field_with_last_tie_pipe_impl(event)
}

fn longest_field_with_last_tie_pipe_impl(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.lines().last().unwrap_or_default();
                text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
