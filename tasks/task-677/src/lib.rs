#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub enum Event<'a> {
    Data(&'a str),
    Heartbeat,
    End,
}

pub fn leading_marks_removed_pipe(event: Event<'_>) -> Option<usize> {
    None
}
