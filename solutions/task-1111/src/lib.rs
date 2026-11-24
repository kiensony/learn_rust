// rust-track: handwritten task-1111
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1111;

pub trait Source {
    fn sample(&self) -> i16;
}
pub struct Constant(pub i8);
pub struct Negate(pub i8);
impl Source for Constant {
    fn sample(&self) -> i16 {
        self.0.into()
    }
}
impl Source for Negate {
    fn sample(&self) -> i16 {
        -i16::from(self.0)
    }
}

pub fn sample_all(inputs: &[&dyn Source]) -> Vec<i16> {
    inputs.iter().map(|s| s.sample()).collect()
}
