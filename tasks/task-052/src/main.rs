fn main() {
    let visitor = std::env::args().nth(1).unwrap();
    println!("{visitor}");
}
