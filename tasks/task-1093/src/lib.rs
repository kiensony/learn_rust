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

pub fn leading_marks_removed_hash(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_start_matches('#')
    })
    .chars()
    .count()
}
