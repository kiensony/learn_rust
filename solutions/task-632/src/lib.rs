// rust-track: handwritten task-632
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 632;

pub struct Panel {
    pub width: u16,
    pub height: u16,
}

pub fn border(panel: Panel) -> u32 {
    2 * (u32::from(panel.width) + u32::from(panel.height))
}
