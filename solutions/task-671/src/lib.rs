// rust-track: handwritten task-671
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 671;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn prefix_before_final_mark_slash(event: Event<'_>) -> Option<usize> {
    prefix_before_final_mark_slash_impl(event)
}

fn prefix_before_final_mark_slash_impl(event: Event<'_>) -> Option<usize> {
    match event {
        Event::Data(text) => Some(
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.rsplit_once('/').map_or("", |(a, _)| a)
            })
            .len(),
        ),
        Event::Heartbeat => Some(0),
        Event::End => None,
    }
}
