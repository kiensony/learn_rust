// rust-track: handwritten task-1066
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1066;

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

pub fn remainder_after_first_mark_slash(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    })
    .len()
}
