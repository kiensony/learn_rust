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

pub fn marks_removed_at_both_ends_equals(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_matches('=')
    })
    .chars()
    .count()
}
