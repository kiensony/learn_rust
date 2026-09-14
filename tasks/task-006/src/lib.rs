#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(w: u32, h: u32) -> Option<Self> {
        let _ = (w, h);
        None
    }

    pub fn area(&self) -> u32 {
        0
    }

    pub fn can_hold(&self, other: &Rect) -> bool {
        let _ = other;
        false
    }
}
