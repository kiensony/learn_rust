// rust-track: handwritten task-170
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 170;

pub fn instruction(signal: char) -> &'static str {
    match signal {
        'R' => "stop",
        'G' => "go",
        _ => "wait",
    }
}
