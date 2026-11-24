fn main() {
    let room = std::env::var("TRACK_ROOM").unwrap_or_default();
    println!("Room: {room}");
}
