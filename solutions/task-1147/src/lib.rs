// rust-track: handwritten task-1147
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1147;

pub trait Filter {
    fn apply(&self, n: i16) -> i16;
}
pub struct Offset(pub i16);
pub struct Clamp(pub i16);
impl Filter for Offset {
    fn apply(&self, n: i16) -> i16 {
        n.saturating_add(self.0)
    }
}
impl Filter for Clamp {
    fn apply(&self, n: i16) -> i16 {
        n.min(self.0)
    }
}

pub fn apply_all(mut value: i16, steps: &[&dyn Filter]) -> i16 {
    for step in steps {
        value = step.apply(value);
    }
    value
}
