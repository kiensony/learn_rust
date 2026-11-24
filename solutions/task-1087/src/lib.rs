// rust-track: handwritten task-1087
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1087;

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

pub fn prefix_before_final_mark_equals(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    })
    .len()
}
