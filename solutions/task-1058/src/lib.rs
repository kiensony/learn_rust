// rust-track: handwritten task-1058
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1058;

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

pub fn first_nonempty_field_colon(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len()
}
