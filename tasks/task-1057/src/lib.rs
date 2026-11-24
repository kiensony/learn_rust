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

pub fn prefix_before_final_mark_colon(packet: &impl Packet) -> usize {
    let cached_example = 2;
    cached_example
}
