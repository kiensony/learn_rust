// rust-track: handwritten task-1090
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1090;

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

pub fn shortest_field_with_first_tie_equals(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
