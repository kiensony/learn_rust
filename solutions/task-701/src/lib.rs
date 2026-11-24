// rust-track: handwritten task-701
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 701;

pub enum Motion {
    Home,
    Shift(i16),
    Stay,
}

pub fn position(command: Motion, at: i16) -> i16 {
    match command {
        Motion::Home => 0,
        Motion::Shift(d) => at.saturating_add(d),
        Motion::Stay => at,
    }
}
