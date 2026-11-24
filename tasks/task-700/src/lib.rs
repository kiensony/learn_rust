#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn remainder_after_first_mark_hash(event: Event<'_>) -> Option<usize> {
    None
}
