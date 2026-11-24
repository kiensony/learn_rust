#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    Up,
    Pressed,
    Held,
}

pub fn step(state: Button, down: bool) -> Button {
    if down { Button::Pressed } else { Button::Up }
}
