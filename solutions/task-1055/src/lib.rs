// rust-track: handwritten task-1055
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1055;

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

pub fn closing_marks_removed_colon(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    (text.trim_end_matches(':')).len()
}
