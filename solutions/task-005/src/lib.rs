// rust-track: handwritten task-005
pub const TRACK_PROOF: u32 = 0x52535400 | 5;

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}
