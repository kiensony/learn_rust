// rust-track: handwritten task-1060
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1060;

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

pub fn shortest_field_with_first_tie_colon(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len()
}
