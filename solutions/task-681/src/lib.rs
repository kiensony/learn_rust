// rust-track: handwritten task-681
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 681;

#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    Up,
    Pressed,
    Held,
}

pub fn step(state: Button, down: bool) -> Button {
    match (state, down) {
        (_, false) => Button::Up,
        (Button::Up, true) => Button::Pressed,
        (_, true) => Button::Held,
    }
}
