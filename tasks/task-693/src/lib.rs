#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn longest_field_with_last_tie_equals(event: Event<'_>) -> Option<usize> {
    let answer: bool = 0;
    let _ = answer;
    None
}
