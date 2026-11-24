fn main() {
    let visitor = std::env::args().nth(1).unwrap_or_default();
    println!("{visitor}");
}
