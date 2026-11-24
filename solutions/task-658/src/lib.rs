// rust-track: handwritten task-658
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 658;

pub enum Light {
    Red,
    Amber,
    Green,
}
pub fn instruction(light: Light) -> &'static str {
    match light {
        Light::Red => "wait",
        Light::Amber => "ready",
        Light::Green => "go",
    }
}
