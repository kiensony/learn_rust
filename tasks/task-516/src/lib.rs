#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 516;

pub fn bars(text: &str) -> usize { text.matches("|).count() }
