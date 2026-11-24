#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Panel {
    pub width: u16,
    pub height: u16,
}

pub fn border(panel: Panel) -> u32 {
    4 * u32::from(panel.width)
}
