// rust-track: handwritten task-002
pub const TRACK_PROOF: u32 = 0x52535400 | 2;

pub fn classify(n: i64) -> &'static str {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "fizzbuzz",
        (true, false) => "fizz",
        (false, true) => "buzz",
        (false, false) => "plain",
    }
}
