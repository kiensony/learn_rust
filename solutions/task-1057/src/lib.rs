// rust-track: handwritten task-1057
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1057;

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
    prefix_before_final_mark_colon_impl(packet)
}

fn prefix_before_final_mark_colon_impl(packet: &impl Packet) -> usize {
    let text = packet.payload().as_ref();
    (text.rsplit_once(':').map_or("", |(a, _)| a)).len()
}
