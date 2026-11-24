#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn marks_removed_at_both_ends_hash(event: Event<'_>) -> Option<usize> {
    None
}
