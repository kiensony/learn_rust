// rust-track: handwritten task-1056
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1056;

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

pub fn marks_removed_at_both_ends_colon(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    (text.trim_matches(':')).len()
}
