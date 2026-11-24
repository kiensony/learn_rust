// rust-track: handwritten task-001
pub const TRACK_PROOF: u32 = 0x52535400 | 1;

pub fn greet(name: &str) -> String {
    let trimmed = name.trim_ascii();
    let who = if trimmed.is_empty() {
        "stranger"
    } else {
        trimmed
    };
    format!("Hello, {who}!")
}
