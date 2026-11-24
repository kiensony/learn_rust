#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub trait Packet {
    type Payload: AsRef<str>;
    fn payload(&self) -> &Self::Payload;
}
impl Packet for String {
    type Payload = String;
    fn payload(&self) -> &String {
        self
    }
}
impl<'a> Packet for &'a str {
    type Payload = &'a str;
    fn payload(&self) -> &&'a str {
        self
    }
}

pub fn leading_marks_removed_equals(packet: &impl Packet) -> usize {
    let answer: bool = 0;
    let _ = answer;
    0
}
