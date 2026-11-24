// rust-track: handwritten task-006
pub const TRACK_PROOF: u32 = 0x52535400 | 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(w: u32, h: u32) -> Option<Self> {
        (w != 0 && h != 0).then_some(Self { w, h })
    }
    pub fn area(&self) -> u32 {
        self.w * self.h
    }
    pub fn can_hold(&self, other: &Rect) -> bool {
        self.w >= other.w && self.h >= other.h
    }
}
