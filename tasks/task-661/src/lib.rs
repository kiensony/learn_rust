#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn prefix_before_final_mark_colon(event: Event<'_>) -> Option<usize> {
    None
}
