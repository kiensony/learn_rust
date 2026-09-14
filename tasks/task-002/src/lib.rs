pub const TRACK_PROOF: u32 = 0x52535400 | 2;

pub fn classify(n: i64) -> &'static str {
    let _ = n;
    
    if n % 3 == 0 && n % 5 == 0 {
        "fizzbuzz"
    } else if n % 3 == 0 {
        "fizz"
    } else if n % 5 == 0 {
        "buzz"
    } else {
        "plain"
    }
}
