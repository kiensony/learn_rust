#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn shortest_field_with_first_tie_hash(event: Event<'_>) -> Option<usize> {
    None
}
