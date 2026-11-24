fn main() {
    let ticket = std::env::args().next().unwrap_or_default();
    println!("{ticket}");
}
