// rust-track: handwritten task-1097
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1097;

pub trait Span {
    fn span(&self) -> u32;
}
pub struct Length(pub u32);
pub struct Interval(pub u32, pub u32);
impl Span for Length {
    fn span(&self) -> u32 {
        self.0
    }
}
impl Span for Interval {
    fn span(&self) -> u32 {
        self.0.abs_diff(self.1)
    }
}

pub fn total_span<T: Span>(items: &[T]) -> u64 {
    items.iter().map(|item| u64::from(item.span())).sum()
}
