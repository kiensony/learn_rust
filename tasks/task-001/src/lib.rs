pub const TRACK_PROOF: u32 = 0x52535400 | 1;

pub fn greet(name: &str) -> String {
    let _ = name;
    let name = String::from(name).trim().to_string();

    if name == "" {
        return String::from("Hello, stranger!");
    }

    let greeting = String::from("Hello");

    return format!("{}, {}!", &greeting, &name);
}
