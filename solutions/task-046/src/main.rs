fn main() {
    let ticket = std::env::args().nth(1).unwrap_or_default();
    println!("{ticket}");
}
